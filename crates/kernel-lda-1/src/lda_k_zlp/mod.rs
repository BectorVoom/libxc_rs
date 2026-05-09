//! LDA_K_ZLP kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=18 lines
//!   exc: shared=0, delta=18, outputs=1
//!   vxc: shared=18, delta=6, outputs=2
//!   fxc: shared=24, delta=7, outputs=3
//!   kxc: shared=31, delta=6, outputs=4
//!   lxc: shared=37, delta=3, outputs=5
//! pol: preamble=33 lines
//!   exc: shared=0, delta=33, outputs=1
//!   vxc: shared=33, delta=24, outputs=3
//!   fxc: shared=57, delta=47, outputs=6
//!   kxc: shared=104, delta=72, outputs=10
//!   lxc: shared=176, delta=76, outputs=15

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
