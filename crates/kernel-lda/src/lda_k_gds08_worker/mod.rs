//! LDA_K_GDS08_WORKER kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=9 lines
//!   exc: shared=0, delta=9, outputs=1
//!   vxc: shared=9, delta=4, outputs=2
//!   fxc: shared=13, delta=4, outputs=3
//!   kxc: shared=17, delta=3, outputs=4
//!   lxc: shared=20, delta=4, outputs=5
//! pol: preamble=30 lines
//!   exc: shared=0, delta=30, outputs=1
//!   vxc: shared=30, delta=42, outputs=3
//!   fxc: shared=72, delta=85, outputs=6
//!   kxc: shared=157, delta=167, outputs=10
//!   lxc: shared=324, delta=275, outputs=15

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
