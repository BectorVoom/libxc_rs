//! LDA_C_VWN_3 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=74 lines
//!   exc: shared=0, delta=74, outputs=1
//!   vxc: shared=74, delta=120, outputs=2
//!   fxc: shared=194, delta=217, outputs=3
//!   kxc: shared=411, delta=360, outputs=4
//!   lxc: shared=771, delta=196, outputs=5
//! pol: preamble=98 lines
//!   exc: shared=0, delta=98, outputs=1
//!   vxc: shared=98, delta=173, outputs=3
//!   fxc: shared=271, delta=373, outputs=6
//!   kxc: shared=644, delta=751, outputs=10
//!   lxc: shared=1395, delta=752, outputs=15

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
