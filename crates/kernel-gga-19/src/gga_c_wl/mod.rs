//! GGA_C_WL kernel -- incremental derivative structure.

//! unpol: preamble=16 lines
//!   exc: shared=0, delta=16, outputs=1
//!   vxc: shared=16, delta=13, outputs=3
//!   fxc: shared=29, delta=29, outputs=6
//!   kxc: shared=58, delta=49, outputs=10
//!   lxc: shared=107, delta=15, outputs=15
//! pol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=40, outputs=6
//!   fxc: shared=68, delta=143, outputs=21
//!   kxc: shared=211, delta=513, outputs=56
//!   lxc: shared=724, delta=1150, outputs=126

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
