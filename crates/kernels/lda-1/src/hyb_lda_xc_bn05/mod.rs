//! HYB_LDA_XC_BN05 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=72, outputs=2
//!   fxc: shared=157, delta=95, outputs=3
//!   kxc: shared=252, delta=93, outputs=4
//!   lxc: shared=345, delta=58, outputs=5
//! pol: preamble=161 lines
//!   exc: shared=0, delta=161, outputs=1
//!   vxc: shared=161, delta=255, outputs=3
//!   fxc: shared=416, delta=531, outputs=6
//!   kxc: shared=947, delta=818, outputs=10
//!   lxc: shared=1765, delta=875, outputs=15

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
