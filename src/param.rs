
use serialize::json;

use builder::Builder;
use coercers::Coercer;
use ValicoResult;

pub struct Param {
	pub name: String,
	pub coercer: Option<Box<Coercer>>,
	pub nest: Option<Builder>,
	pub description: Option<String>
	// pub validators
	// pub allow_nul
}

impl Param {

	pub fn new(name: &str) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: None,
			nest: None
		}
	}

	pub fn new_with_coercer(name: &str, coercer: Box<Coercer>) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: Some(coercer),
			nest: None
		}
	}

	pub fn new_with_nest(name: &str, coercer: Box<Coercer>, nest: Builder) -> Param {
		Param {
			name: name.to_string(),
			description: None,
			coercer: Some(coercer),
			nest: Some(nest)
		}
	}

	pub fn build(name: &str, build_def: |param: &mut Param|) -> Param {
		let mut param = Param::new(name);
		build_def(&mut param);

		param
	}

	pub fn desc(&mut self, description: &str) {
		self.description = Some(description.to_string());
	}

	pub fn coerce(&mut self, coercer: Box<Coercer>) {
		self.coercer = Some(coercer);
	}

	pub fn nest(&mut self, nest_def: |&mut Builder|) {
		self.nest = Some(Builder::build(nest_def));
	}

	pub fn process(&self, val: &mut json::Json) -> ValicoResult<Option<json::Json>> {
		match self.coercer.as_ref() {
			Some(coercer) => coercer.coerce(val, self.nest.as_ref()),
			None => Ok(None)
		}
	}
}