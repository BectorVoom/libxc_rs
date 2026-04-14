//! MGGA_C_M06L kernel -- incremental derivative structure.

//! unpol: preamble=202 lines
//!   exc: shared=0, delta=202, outputs=1
//!   vxc: shared=202, delta=240, outputs=5
//!   fxc: shared=442, delta=483, outputs=15
//!   kxc: shared=925, delta=900, outputs=35
//!   lxc: shared=1825, delta=477, outputs=70
//! pol: preamble=295 lines
//!   exc: shared=0, delta=295, outputs=1
//!   vxc: shared=295, delta=518, outputs=10
//!   fxc: shared=813, delta=1525, outputs=55
//!   kxc: shared=2338, delta=3972, outputs=220
//!   lxc: shared=6310, delta=3682, outputs=715

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
