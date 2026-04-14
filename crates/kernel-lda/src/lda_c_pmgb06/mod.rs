//! LDA_C_PMGB06 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=78, outputs=2
//!   fxc: shared=194, delta=111, outputs=3
//!   kxc: shared=305, delta=134, outputs=4
//!   lxc: shared=439, delta=79, outputs=5
//! pol: preamble=191 lines
//!   exc: shared=0, delta=191, outputs=1
//!   vxc: shared=191, delta=310, outputs=3
//!   fxc: shared=501, delta=937, outputs=6
//!   kxc: shared=1438, delta=2589, outputs=10
//!   lxc: shared=4027, delta=5774, outputs=15

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
