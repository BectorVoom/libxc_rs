//! MGGA_X_2D_PRP10 kernel -- incremental derivative structure.

//! unpol: preamble=22 lines
//!   vxc: shared=0, delta=22, outputs=1
//!   fxc: shared=22, delta=35, outputs=5
//!   kxc: shared=57, delta=124, outputs=15
//!   lxc: shared=181, delta=258, outputs=35
//! pol: preamble=40 lines
//!   vxc: shared=0, delta=40, outputs=2
//!   fxc: shared=40, delta=73, outputs=19
//!   kxc: shared=113, delta=297, outputs=100
//!   lxc: shared=410, delta=763, outputs=385

pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
