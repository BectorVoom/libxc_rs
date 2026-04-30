//! LDA_X_REL kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=31 lines
//!   exc: shared=0, delta=31, outputs=1
//!   vxc: shared=31, delta=13, outputs=2
//!   fxc: shared=44, delta=14, outputs=3
//!   kxc: shared=58, delta=15, outputs=4
//!   lxc: shared=73, delta=8, outputs=5
//! pol: preamble=47 lines
//!   exc: shared=0, delta=47, outputs=1
//!   vxc: shared=47, delta=48, outputs=3
//!   fxc: shared=95, delta=83, outputs=6
//!   kxc: shared=178, delta=136, outputs=10
//!   lxc: shared=314, delta=168, outputs=15

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
