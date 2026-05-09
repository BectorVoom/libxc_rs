//! LDA_X_SLOC kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=7 lines
//!   exc: shared=0, delta=7, outputs=1
//!   vxc: shared=7, delta=1, outputs=2
//!   fxc: shared=8, delta=4, outputs=3
//!   kxc: shared=12, delta=4, outputs=4
//!   lxc: shared=16, delta=3, outputs=5
//! pol: preamble=19 lines
//!   exc: shared=0, delta=19, outputs=1
//!   vxc: shared=19, delta=23, outputs=3
//!   fxc: shared=42, delta=47, outputs=6
//!   kxc: shared=89, delta=89, outputs=10
//!   lxc: shared=178, delta=133, outputs=15

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
