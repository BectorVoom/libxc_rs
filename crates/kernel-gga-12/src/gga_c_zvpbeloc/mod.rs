//! GGA_C_ZVPBELOC kernel -- incremental derivative structure.

//! unpol: preamble=94 lines
//!   exc: shared=0, delta=94, outputs=1
//!   vxc: shared=94, delta=109, outputs=3
//!   fxc: shared=203, delta=210, outputs=6
//!   kxc: shared=413, delta=370, outputs=10
//!   lxc: shared=783, delta=221, outputs=15
//! pol: preamble=132 lines
//!   exc: shared=0, delta=132, outputs=1
//!   vxc: shared=132, delta=207, outputs=6
//!   fxc: shared=339, delta=687, outputs=21
//!   kxc: shared=1026, delta=2220, outputs=56
//!   lxc: shared=3246, delta=4746, outputs=126

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
