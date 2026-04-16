//! MGGA_C_LTAPW kernel -- incremental derivative structure.

//! unpol: preamble=39 lines
//!   exc: shared=0, delta=39, outputs=1
//!   vxc: shared=39, delta=68, outputs=5
//!   fxc: shared=107, delta=230, outputs=15
//!   kxc: shared=337, delta=656, outputs=35
//!   lxc: shared=993, delta=832, outputs=70
//! pol: preamble=72 lines
//!   exc: shared=0, delta=72, outputs=1
//!   vxc: shared=72, delta=226, outputs=10
//!   fxc: shared=298, delta=1428, outputs=55
//!   kxc: shared=1726, delta=7442, outputs=220
//!   lxc: shared=9168, delta=11347, outputs=715

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
