//! LDA_C_2D_PRM kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=25 lines
//!   exc: shared=0, delta=25, outputs=1
//!   vxc: shared=25, delta=20, outputs=2
//!   fxc: shared=45, delta=32, outputs=3
//!   kxc: shared=77, delta=43, outputs=4
//!   lxc: shared=120, delta=18, outputs=5
//! pol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=21, outputs=3
//!   fxc: shared=47, delta=34, outputs=6
//!   kxc: shared=81, delta=46, outputs=10
//!   lxc: shared=127, delta=22, outputs=15

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
