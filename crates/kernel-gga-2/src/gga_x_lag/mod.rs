//! GGA_X_LAG kernel -- incremental derivative structure.

//! unpol: preamble=25 lines
//!   exc: shared=0, delta=25, outputs=1
//!   vxc: shared=25, delta=24, outputs=3
//!   fxc: shared=49, delta=51, outputs=6
//!   kxc: shared=100, delta=74, outputs=10
//!   lxc: shared=174, delta=50, outputs=15
//! pol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=73, outputs=6
//!   fxc: shared=124, delta=190, outputs=21
//!   kxc: shared=314, delta=422, outputs=56
//!   lxc: shared=736, delta=648, outputs=126

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
