//! HYB_MGGA_X_JS18 kernel -- incremental derivative structure.

//! unpol: preamble=242 lines
//!   exc: shared=0, delta=242, outputs=1
//!   vxc: shared=242, delta=340, outputs=5
//!   fxc: shared=582, delta=731, outputs=15
//!   kxc: shared=1313, delta=1495, outputs=35
//!   lxc: shared=2808, delta=1810, outputs=70
//! pol: preamble=471 lines
//!   exc: shared=0, delta=471, outputs=1
//!   vxc: shared=471, delta=909, outputs=10
//!   fxc: shared=1380, delta=2568, outputs=55
//!   kxc: shared=3948, delta=5216, outputs=220
//!   lxc: shared=9164, delta=8151, outputs=715

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
