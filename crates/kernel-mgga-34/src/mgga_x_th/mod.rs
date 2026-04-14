//! MGGA_X_TH kernel -- incremental derivative structure.

//! unpol: preamble=22 lines
//!   exc: shared=0, delta=22, outputs=1
//!   vxc: shared=22, delta=14, outputs=5
//!   fxc: shared=36, delta=19, outputs=15
//!   kxc: shared=55, delta=30, outputs=35
//!   lxc: shared=85, delta=40, outputs=70
//! pol: preamble=47 lines
//!   exc: shared=0, delta=47, outputs=1
//!   vxc: shared=47, delta=71, outputs=10
//!   fxc: shared=118, delta=166, outputs=55
//!   kxc: shared=284, delta=405, outputs=220
//!   lxc: shared=689, delta=794, outputs=715

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
