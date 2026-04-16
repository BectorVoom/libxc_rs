//! HYB_GGA_XC_CASE21 kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=72, outputs=3
//!   fxc: shared=142, delta=111, outputs=6
//!   kxc: shared=253, delta=162, outputs=10
//!   lxc: shared=415, delta=84, outputs=15
//! pol: preamble=132 lines
//!   exc: shared=0, delta=132, outputs=1
//!   vxc: shared=132, delta=189, outputs=6
//!   fxc: shared=321, delta=467, outputs=21
//!   kxc: shared=788, delta=1094, outputs=56
//!   lxc: shared=1882, delta=1310, outputs=126

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
