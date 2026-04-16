//! MGGA_X_MCML kernel -- incremental derivative structure.

//! unpol: preamble=92 lines
//!   exc: shared=0, delta=92, outputs=1
//!   vxc: shared=92, delta=187, outputs=5
//!   fxc: shared=279, delta=485, outputs=15
//!   kxc: shared=764, delta=1031, outputs=35
//!   lxc: shared=1795, delta=1830, outputs=70
//! pol: preamble=167 lines
//!   exc: shared=0, delta=167, outputs=1
//!   vxc: shared=167, delta=387, outputs=10
//!   fxc: shared=554, delta=1046, outputs=55
//!   kxc: shared=1600, delta=2306, outputs=220
//!   lxc: shared=3906, delta=4298, outputs=715

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
