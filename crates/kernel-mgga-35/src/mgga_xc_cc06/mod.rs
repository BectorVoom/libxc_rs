//! MGGA_XC_CC06 kernel -- incremental derivative structure.

//! unpol: preamble=42 lines
//!   exc: shared=0, delta=42, outputs=1
//!   vxc: shared=42, delta=43, outputs=5
//!   fxc: shared=85, delta=75, outputs=15
//!   kxc: shared=160, delta=88, outputs=35
//!   lxc: shared=248, delta=75, outputs=70
//! pol: preamble=96 lines
//!   exc: shared=0, delta=96, outputs=1
//!   vxc: shared=96, delta=136, outputs=10
//!   fxc: shared=232, delta=334, outputs=55
//!   kxc: shared=566, delta=742, outputs=220
//!   lxc: shared=1308, delta=1205, outputs=715

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
