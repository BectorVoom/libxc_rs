//! MGGA_X_M11 kernel -- incremental derivative structure.

//! unpol: preamble=149 lines
//!   exc: shared=0, delta=149, outputs=1
//!   vxc: shared=149, delta=148, outputs=5
//!   fxc: shared=297, delta=230, outputs=15
//!   kxc: shared=527, delta=299, outputs=35
//!   lxc: shared=826, delta=348, outputs=70
//! pol: preamble=275 lines
//!   exc: shared=0, delta=275, outputs=1
//!   vxc: shared=275, delta=315, outputs=10
//!   fxc: shared=590, delta=704, outputs=55
//!   kxc: shared=1294, delta=1316, outputs=220
//!   lxc: shared=2610, delta=1708, outputs=715

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
