//! LDA_X kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=10 lines
//!   exc: shared=0, delta=10, outputs=1
//!   vxc: shared=10, delta=4, outputs=2
//!   fxc: shared=14, delta=2, outputs=3
//!   kxc: shared=16, delta=3, outputs=4
//!   lxc: shared=19, delta=2, outputs=5
//! pol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=39, outputs=3
//!   fxc: shared=65, delta=67, outputs=6
//!   kxc: shared=132, delta=112, outputs=10
//!   lxc: shared=244, delta=126, outputs=15

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
