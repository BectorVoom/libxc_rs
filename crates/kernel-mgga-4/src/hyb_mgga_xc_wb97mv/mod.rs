//! HYB_MGGA_XC_WB97MV kernel -- incremental derivative structure.

//! unpol: preamble=192 lines
//!   exc: shared=0, delta=192, outputs=1
//!   vxc: shared=192, delta=228, outputs=5
//!   fxc: shared=420, delta=431, outputs=15
//!   kxc: shared=851, delta=756, outputs=35
//!   lxc: shared=1607, delta=995, outputs=70
//! pol: preamble=332 lines
//!   exc: shared=0, delta=332, outputs=1
//!   vxc: shared=332, delta=564, outputs=10
//!   fxc: shared=896, delta=1617, outputs=55
//!   kxc: shared=2513, delta=4527, outputs=220
//!   lxc: shared=7040, delta=8976, outputs=715

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
