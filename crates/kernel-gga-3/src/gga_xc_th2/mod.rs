//! GGA_XC_TH2 kernel -- incremental derivative structure.

//! unpol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=26, outputs=3
//!   fxc: shared=77, delta=23, outputs=6
//!   kxc: shared=100, delta=24, outputs=10
//!   lxc: shared=124, delta=12, outputs=15
//! pol: preamble=96 lines
//!   exc: shared=0, delta=96, outputs=1
//!   vxc: shared=96, delta=99, outputs=6
//!   fxc: shared=195, delta=259, outputs=21
//!   kxc: shared=454, delta=400, outputs=56
//!   lxc: shared=854, delta=446, outputs=126

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
