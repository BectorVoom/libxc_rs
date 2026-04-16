//! MGGA_C_RMGGAC kernel -- incremental derivative structure.

//! unpol: preamble=104 lines
//!   exc: shared=0, delta=104, outputs=1
//!   vxc: shared=104, delta=156, outputs=5
//!   fxc: shared=260, delta=589, outputs=15
//!   kxc: shared=849, delta=2001, outputs=35
//!   lxc: shared=2850, delta=1805, outputs=70
//! pol: preamble=161 lines
//!   exc: shared=0, delta=161, outputs=1
//!   vxc: shared=161, delta=340, outputs=10
//!   fxc: shared=501, delta=1988, outputs=55
//!   kxc: shared=2489, delta=13097, outputs=220
//!   lxc: shared=15586, delta=41642, outputs=715

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
