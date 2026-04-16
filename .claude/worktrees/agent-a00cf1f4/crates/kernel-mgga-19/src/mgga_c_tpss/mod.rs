//! MGGA_C_TPSS kernel -- incremental derivative structure.

//! unpol: preamble=284 lines
//!   exc: shared=0, delta=284, outputs=1
//!   vxc: shared=284, delta=344, outputs=5
//!   fxc: shared=628, delta=702, outputs=15
//!   kxc: shared=1330, delta=1227, outputs=35
//!   lxc: shared=2557, delta=935, outputs=70
//! pol: preamble=363 lines
//!   exc: shared=0, delta=363, outputs=1
//!   vxc: shared=363, delta=762, outputs=10
//!   fxc: shared=1125, delta=2534, outputs=55
//!   kxc: shared=3659, delta=8128, outputs=220
//!   lxc: shared=11787, delta=19212, outputs=715

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
