//! LDA_C_ML1 kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=9, outputs=2
//!   fxc: shared=44, delta=16, outputs=3
//!   kxc: shared=60, delta=17, outputs=4
//!   lxc: shared=77, delta=6, outputs=5
//! pol: preamble=58 lines
//!   exc: shared=0, delta=58, outputs=1
//!   vxc: shared=58, delta=101, outputs=3
//!   fxc: shared=159, delta=358, outputs=6
//!   kxc: shared=517, delta=1221, outputs=10
//!   lxc: shared=1738, delta=2438, outputs=15

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
