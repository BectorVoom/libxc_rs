//! LDA_XC_KSDT kernel — incremental derivative structure.
//!
//!
//! unpol: preamble=185 lines
//!   exc: shared=0, delta=185, outputs=1
//!   vxc: shared=185, delta=223, outputs=2
//!   fxc: shared=408, delta=406, outputs=3
//!   kxc: shared=814, delta=783, outputs=4
//!   lxc: shared=1597, delta=366, outputs=5
//! pol: preamble=242 lines
//!   exc: shared=0, delta=242, outputs=1
//!   vxc: shared=242, delta=505, outputs=3
//!   fxc: shared=747, delta=1479, outputs=6
//!   kxc: shared=2226, delta=4214, outputs=10
//!   lxc: shared=6440, delta=7439, outputs=15

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
