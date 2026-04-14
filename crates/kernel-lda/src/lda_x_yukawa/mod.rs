//! LDA_X_YUKAWA kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=61 lines
//!   exc: shared=0, delta=61, outputs=1
//!   vxc: shared=61, delta=41, outputs=2
//!   fxc: shared=102, delta=28, outputs=3
//!   kxc: shared=130, delta=45, outputs=4
//!   lxc: shared=175, delta=23, outputs=5
//! pol: preamble=122 lines
//!   exc: shared=0, delta=122, outputs=1
//!   vxc: shared=122, delta=197, outputs=3
//!   fxc: shared=319, delta=395, outputs=6
//!   kxc: shared=714, delta=613, outputs=10
//!   lxc: shared=1327, delta=531, outputs=15

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
