//! GGA_X_BPCCAC kernel -- incremental derivative structure.

//! unpol: preamble=60 lines
//!   exc: shared=0, delta=60, outputs=1
//!   vxc: shared=60, delta=59, outputs=3
//!   fxc: shared=119, delta=111, outputs=6
//!   kxc: shared=230, delta=190, outputs=10
//!   lxc: shared=420, delta=108, outputs=15
//! pol: preamble=97 lines
//!   exc: shared=0, delta=97, outputs=1
//!   vxc: shared=97, delta=113, outputs=6
//!   fxc: shared=210, delta=256, outputs=21
//!   kxc: shared=466, delta=477, outputs=56
//!   lxc: shared=943, delta=437, outputs=126

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
