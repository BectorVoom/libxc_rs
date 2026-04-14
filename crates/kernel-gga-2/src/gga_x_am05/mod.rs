//! GGA_X_AM05 kernel -- incremental derivative structure.

//! unpol: preamble=65 lines
//!   exc: shared=0, delta=65, outputs=1
//!   vxc: shared=65, delta=73, outputs=3
//!   fxc: shared=138, delta=123, outputs=6
//!   kxc: shared=261, delta=228, outputs=10
//!   lxc: shared=489, delta=138, outputs=15
//! pol: preamble=107 lines
//!   exc: shared=0, delta=107, outputs=1
//!   vxc: shared=107, delta=153, outputs=6
//!   fxc: shared=260, delta=264, outputs=21
//!   kxc: shared=524, delta=527, outputs=56
//!   lxc: shared=1051, delta=530, outputs=126

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
