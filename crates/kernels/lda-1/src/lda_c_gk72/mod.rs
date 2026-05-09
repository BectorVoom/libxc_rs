//! LDA_C_GK72 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=24 lines
//!   exc: shared=0, delta=24, outputs=1
//!   vxc: shared=24, delta=10, outputs=2
//!   fxc: shared=34, delta=16, outputs=3
//!   kxc: shared=50, delta=15, outputs=4
//!   lxc: shared=65, delta=10, outputs=5
//! pol: preamble=25 lines
//!   exc: shared=0, delta=25, outputs=1
//!   vxc: shared=25, delta=11, outputs=3
//!   fxc: shared=36, delta=18, outputs=6
//!   kxc: shared=54, delta=18, outputs=10
//!   lxc: shared=72, delta=14, outputs=15

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
