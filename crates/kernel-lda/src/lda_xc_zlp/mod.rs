//! LDA_XC_ZLP kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=6 lines
//!   exc: shared=0, delta=6, outputs=1
//!   vxc: shared=6, delta=6, outputs=2
//!   fxc: shared=12, delta=6, outputs=3
//!   kxc: shared=18, delta=6, outputs=4
//!   lxc: shared=24, delta=3, outputs=5
//! pol: preamble=7 lines
//!   exc: shared=0, delta=7, outputs=1
//!   vxc: shared=7, delta=7, outputs=3
//!   fxc: shared=14, delta=8, outputs=6
//!   kxc: shared=22, delta=9, outputs=10
//!   lxc: shared=31, delta=7, outputs=15

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
