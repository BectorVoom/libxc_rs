//! MGGA_C_CCALDA kernel -- incremental derivative structure.

//! unpol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=67, outputs=5
//!   fxc: shared=118, delta=152, outputs=15
//!   kxc: shared=270, delta=255, outputs=35
//!   lxc: shared=525, delta=139, outputs=70
//! pol: preamble=107 lines
//!   exc: shared=0, delta=107, outputs=1
//!   vxc: shared=107, delta=230, outputs=10
//!   fxc: shared=337, delta=1075, outputs=55
//!   kxc: shared=1412, delta=5024, outputs=220
//!   lxc: shared=6436, delta=8115, outputs=715

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
