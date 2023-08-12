use std::io;

use async_trait::async_trait;
use futures::prelude::*;
use futures::{AsyncRead, AsyncWrite};
use libp2p::request_response::{self, ProtocolName};

// Simple file exchange protocol

#[derive(Debug, Clone)]
pub struct FileExchangeProtocol();
#[derive(Clone)]
pub struct FileExchangeCodec();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRequest {
    pub file_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileResponse {
    pub file_body: Vec<u8>,
}

const FILE_ID_MAX_SIZE: u64 = 1024;
const FILE_BODY_MAX_SIZE: u64 = 500 * 1024 * 1024;

impl ProtocolName for FileExchangeProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/universal-connectivity-file/1".as_bytes()
    }
}

#[async_trait]
impl request_response::Codec for FileExchangeCodec {
    type Protocol = FileExchangeProtocol;
    type Request = FileRequest;
    type Response = FileResponse;

    async fn read_request<T>(
        &mut self,
        _: &FileExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut file_id = String::new();
        io.take(FILE_ID_MAX_SIZE)
            .read_to_string(&mut file_id)
            .await?;
        if file_id.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(FileRequest { file_id })
    }

    async fn read_response<T>(
        &mut self,
        _: &FileExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut file_body = Vec::new();
        io.take(FILE_BODY_MAX_SIZE)
            .read_to_end(&mut file_body)
            .await
            .unwrap();

        Ok(FileResponse { file_body })
    }

    async fn write_request<T>(
        &mut self,
        _: &FileExchangeProtocol,
        io: &mut T,
        FileRequest { file_id }: FileRequest,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        io.write_all(file_id.as_bytes()).await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &FileExchangeProtocol,
        io: &mut T,
        FileResponse { file_body }: FileResponse,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        io.write_all(file_body.as_slice()).await?;

        Ok(())
    }
}
