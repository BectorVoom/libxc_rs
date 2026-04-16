//! MGGA_X_REGTM kernel -- incremental derivative structure.

//! unpol: preamble=87 lines
//!   exc: shared=0, delta=87, outputs=1
//!   vxc: shared=87, delta=126, outputs=5
//!   fxc: shared=213, delta=342, outputs=15
//!   kxc: shared=555, delta=1017, outputs=35
//!   lxc: shared=1572, delta=1384, outputs=70
//! pol: preamble=157 lines
//!   exc: shared=0, delta=157, outputs=1
//!   vxc: shared=157, delta=250, outputs=10
//!   fxc: shared=407, delta=747, outputs=55
//!   kxc: shared=1154, delta=2248, outputs=220
//!   lxc: shared=3402, delta=3268, outputs=715

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
