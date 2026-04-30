//! MGGA_X_LTA kernel -- incremental derivative structure.

//! unpol: preamble=23 lines
//!   exc: shared=0, delta=23, outputs=1
//!   vxc: shared=23, delta=12, outputs=5
//!   fxc: shared=35, delta=20, outputs=15
//!   kxc: shared=55, delta=34, outputs=35
//!   lxc: shared=89, delta=45, outputs=70
//! pol: preamble=43 lines
//!   exc: shared=0, delta=43, outputs=1
//!   vxc: shared=43, delta=53, outputs=10
//!   fxc: shared=96, delta=147, outputs=55
//!   kxc: shared=243, delta=334, outputs=220
//!   lxc: shared=577, delta=713, outputs=715

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
