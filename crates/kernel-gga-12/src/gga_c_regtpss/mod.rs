//! GGA_C_REGTPSS kernel -- incremental derivative structure.

//! unpol: preamble=80 lines
//!   exc: shared=0, delta=80, outputs=1
//!   vxc: shared=80, delta=115, outputs=3
//!   fxc: shared=195, delta=249, outputs=6
//!   kxc: shared=444, delta=521, outputs=10
//!   lxc: shared=965, delta=400, outputs=15
//! pol: preamble=114 lines
//!   exc: shared=0, delta=114, outputs=1
//!   vxc: shared=114, delta=220, outputs=6
//!   fxc: shared=334, delta=655, outputs=21
//!   kxc: shared=989, delta=2033, outputs=56
//!   lxc: shared=3022, delta=4228, outputs=126

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
