//! LDA_XC_TETER93 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=31 lines
//!   exc: shared=0, delta=31, outputs=1
//!   vxc: shared=31, delta=12, outputs=2
//!   fxc: shared=43, delta=13, outputs=3
//!   kxc: shared=56, delta=17, outputs=4
//!   lxc: shared=73, delta=6, outputs=5
//! pol: preamble=45 lines
//!   exc: shared=0, delta=45, outputs=1
//!   vxc: shared=45, delta=45, outputs=3
//!   fxc: shared=90, delta=86, outputs=6
//!   kxc: shared=176, delta=143, outputs=10
//!   lxc: shared=319, delta=182, outputs=15

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
