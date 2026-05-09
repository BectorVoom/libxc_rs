//! LDA_X_ERF kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=47 lines
//!   exc: shared=0, delta=47, outputs=1
//!   vxc: shared=47, delta=25, outputs=2
//!   fxc: shared=72, delta=21, outputs=3
//!   kxc: shared=93, delta=32, outputs=4
//!   lxc: shared=125, delta=15, outputs=5
//! pol: preamble=93 lines
//!   exc: shared=0, delta=93, outputs=1
//!   vxc: shared=93, delta=139, outputs=3
//!   fxc: shared=232, delta=289, outputs=6
//!   kxc: shared=521, delta=475, outputs=10
//!   lxc: shared=996, delta=433, outputs=15

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
