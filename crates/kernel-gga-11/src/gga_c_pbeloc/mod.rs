//! GGA_C_PBELOC kernel -- incremental derivative structure.

//! unpol: preamble=83 lines
//!   exc: shared=0, delta=83, outputs=1
//!   vxc: shared=83, delta=109, outputs=3
//!   fxc: shared=192, delta=209, outputs=6
//!   kxc: shared=401, delta=365, outputs=10
//!   lxc: shared=766, delta=208, outputs=15
//! pol: preamble=119 lines
//!   exc: shared=0, delta=119, outputs=1
//!   vxc: shared=119, delta=211, outputs=6
//!   fxc: shared=330, delta=688, outputs=21
//!   kxc: shared=1018, delta=2204, outputs=56
//!   lxc: shared=3222, delta=4512, outputs=126

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
