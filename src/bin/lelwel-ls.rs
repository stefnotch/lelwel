#![cfg(feature = "lsp")]

use tokio::sync::RwLock;
use tower_lsp_server::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| lelwel::ide::Backend {
        client,
        cache: RwLock::new(lelwel::ide::Cache::default()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
