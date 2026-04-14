//! MGGA_X_EDMGGA kernel -- incremental derivative structure.

//! unpol: preamble=58 lines
//!   exc: shared=0, delta=58, outputs=1
//!   vxc: shared=58, delta=73, outputs=5
//!   fxc: shared=131, delta=196, outputs=15
//!   kxc: shared=327, delta=500, outputs=35
//!   lxc: shared=827, delta=810, outputs=70
//! pol: preamble=97 lines
//!   exc: shared=0, delta=97, outputs=1
//!   vxc: shared=97, delta=154, outputs=10
//!   fxc: shared=251, delta=454, outputs=55
//!   kxc: shared=705, delta=1254, outputs=220
//!   lxc: shared=1959, delta=2339, outputs=715

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
