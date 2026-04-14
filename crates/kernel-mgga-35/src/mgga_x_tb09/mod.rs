//! MGGA_X_TB09 kernel -- incremental derivative structure.

//! unpol: preamble=41 lines
//!   vxc: shared=0, delta=41, outputs=1
//!   fxc: shared=41, delta=86, outputs=5
//!   kxc: shared=127, delta=362, outputs=15
//!   lxc: shared=489, delta=1208, outputs=35
//! pol: preamble=63 lines
//!   vxc: shared=0, delta=63, outputs=2
//!   fxc: shared=63, delta=172, outputs=19
//!   kxc: shared=235, delta=761, outputs=100
//!   lxc: shared=996, delta=2634, outputs=385

pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
