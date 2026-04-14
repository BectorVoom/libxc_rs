//! MGGA_C_R2SCAN kernel -- incremental derivative structure.

//! unpol: preamble=163 lines
//!   exc: shared=0, delta=163, outputs=1
//!   vxc: shared=163, delta=222, outputs=5
//!   fxc: shared=385, delta=511, outputs=15
//!   kxc: shared=896, delta=993, outputs=35
//!   lxc: shared=1889, delta=783, outputs=70
//! pol: preamble=247 lines
//!   exc: shared=0, delta=247, outputs=1
//!   vxc: shared=247, delta=481, outputs=10
//!   fxc: shared=728, delta=1703, outputs=55
//!   kxc: shared=2431, delta=5783, outputs=220
//!   lxc: shared=8214, delta=13136, outputs=715

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
