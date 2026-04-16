//! MGGA_X_MBRXC_BG kernel -- incremental derivative structure.

//! unpol: preamble=75 lines
//!   exc: shared=0, delta=75, outputs=1
//!   vxc: shared=75, delta=112, outputs=5
//!   fxc: shared=187, delta=460, outputs=15
//!   kxc: shared=647, delta=1731, outputs=35
//!   lxc: shared=2378, delta=3988, outputs=70
//! pol: preamble=127 lines
//!   exc: shared=0, delta=127, outputs=1
//!   vxc: shared=127, delta=235, outputs=10
//!   fxc: shared=362, delta=968, outputs=55
//!   kxc: shared=1330, delta=3790, outputs=220
//!   lxc: shared=5120, delta=8526, outputs=715

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
