//! MGGA_X_2D_PRHG07 kernel -- incremental derivative structure.

//! unpol: preamble=27 lines
//!   exc: shared=0, delta=27, outputs=1
//!   vxc: shared=27, delta=29, outputs=5
//!   fxc: shared=56, delta=126, outputs=15
//!   kxc: shared=182, delta=559, outputs=35
//!   lxc: shared=741, delta=1128, outputs=70
//! pol: preamble=58 lines
//!   exc: shared=0, delta=58, outputs=1
//!   vxc: shared=58, delta=80, outputs=10
//!   fxc: shared=138, delta=340, outputs=55
//!   kxc: shared=478, delta=1485, outputs=220
//!   lxc: shared=1963, delta=4139, outputs=715

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
