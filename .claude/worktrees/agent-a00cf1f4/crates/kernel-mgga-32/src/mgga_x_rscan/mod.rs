//! MGGA_X_RSCAN kernel -- incremental derivative structure.

//! unpol: preamble=104 lines
//!   exc: shared=0, delta=104, outputs=1
//!   vxc: shared=104, delta=129, outputs=5
//!   fxc: shared=233, delta=355, outputs=15
//!   kxc: shared=588, delta=868, outputs=35
//!   lxc: shared=1456, delta=822, outputs=70
//! pol: preamble=205 lines
//!   exc: shared=0, delta=205, outputs=1
//!   vxc: shared=205, delta=344, outputs=10
//!   fxc: shared=549, delta=1395, outputs=55
//!   kxc: shared=1944, delta=5010, outputs=220
//!   lxc: shared=6954, delta=7984, outputs=715

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
