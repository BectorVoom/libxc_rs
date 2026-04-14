//! MGGA_X_VT84 kernel -- incremental derivative structure.

//! unpol: preamble=87 lines
//!   exc: shared=0, delta=87, outputs=1
//!   vxc: shared=87, delta=137, outputs=5
//!   fxc: shared=224, delta=353, outputs=15
//!   kxc: shared=577, delta=972, outputs=35
//!   lxc: shared=1549, delta=1204, outputs=70
//! pol: preamble=160 lines
//!   exc: shared=0, delta=160, outputs=1
//!   vxc: shared=160, delta=273, outputs=10
//!   fxc: shared=433, delta=752, outputs=55
//!   kxc: shared=1185, delta=2052, outputs=220
//!   lxc: shared=3237, delta=3161, outputs=715

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
