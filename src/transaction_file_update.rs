use crate::{
    proto::{self, ToProto, Transaction::TransactionBody_oneof_data},
    crypto::PublicKey,
    transaction::Transaction,
    Client,
    FileId
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;
use chrono::DateTime;
use chrono::Utc;
use protobuf::RepeatedField;


pub struct TransactionFileUpdate {
    id: FileId,
    expiration_time: Option<DateTime<Utc>>,
    keys: Vec<PublicKey>,
    bytes: Vec<u8>,
}

interfaces!(
    TransactionFileUpdate: Any,
    ToProto<TransactionBody_oneof_data>
);

impl TransactionFileUpdate {
    pub fn new(client: &Client, id: FileId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                id,
                expiration_time: None,
                keys: Vec::new(),
                bytes: Vec::new(),
            },
        )
    }
}

impl Transaction<TransactionFileUpdate> {
    #[inline]
    pub fn expires_at(&mut self, expiration: DateTime<Utc>) -> &mut Self {
        self.inner().expiration_time = Some(expiration);
        self
    }

    #[inline]
    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner().keys.push(key);
        self
    }

    #[inline]
    pub fn contents(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.inner().bytes = bytes;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionFileUpdate {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::FileUpdate::FileUpdateTransactionBody::new();

        data.set_fileID(self.id.to_proto()?);

        if let Some(expiration_time) = &self.expiration_time.as_ref() {
            data.set_expirationTime(expiration_time.to_proto()?);
        }

        let mut key_list = proto::BasicTypes::KeyList::new();
        key_list.set_keys(RepeatedField::from_vec(self.keys.iter()
            .map(ToProto::to_proto)
            .collect::<Result<Vec<_>, _>>()?));

        data.set_keys(key_list);

        data.set_contents(self.bytes.clone());

        Ok(TransactionBody_oneof_data::fileUpdate(data))
    }
}