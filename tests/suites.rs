/* Engineer a handshake using each suite. */

#[allow(dead_code)]
mod common;
use common::OpenSSLServer;

#[test]
fn ecdhe_rsa_aes_128_gcm_sha256() {
  let mut server = OpenSSLServer::new(8100);
  server.run();
  server.client()
    .verbose()
    .suite("TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256")
    .expect("Ciphers common between both SSL end points:\nECDHE-RSA-AES128-GCM-SHA256")
    .go();
  server.kill();
}

#[test]
fn ecdhe_rsa_aes_256_gcm_sha384() {
  let mut server = OpenSSLServer::new(8200);
  server.run();
  server.client()
    .verbose()
    .suite("TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384")
    .expect("Ciphers common between both SSL end points:\nECDHE-RSA-AES256-GCM-SHA384")
    .go();
  server.kill();
}