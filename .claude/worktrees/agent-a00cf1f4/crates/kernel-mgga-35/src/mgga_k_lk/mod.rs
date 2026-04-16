//! MGGA_K_LK kernel -- incremental derivative structure.

//! unpol: preamble=68 lines
//!   exc: shared=0, delta=68, outputs=1
//!   vxc: shared=68, delta=51, outputs=5
//!   fxc: shared=119, delta=92, outputs=15
//!   kxc: shared=211, delta=180, outputs=35
//!   lxc: shared=391, delta=164, outputs=70
//! pol: preamble=106 lines
//!   exc: shared=0, delta=106, outputs=1
//!   vxc: shared=106, delta=113, outputs=10
//!   fxc: shared=219, delta=269, outputs=55
//!   kxc: shared=488, delta=630, outputs=220
//!   lxc: shared=1118, delta=950, outputs=715

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
