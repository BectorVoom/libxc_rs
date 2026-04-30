//! GGA_C_OP_PBE kernel -- incremental derivative structure.

//! unpol: preamble=60 lines
//!   exc: shared=0, delta=60, outputs=1
//!   vxc: shared=60, delta=42, outputs=3
//!   fxc: shared=102, delta=93, outputs=6
//!   kxc: shared=195, delta=189, outputs=10
//!   lxc: shared=384, delta=197, outputs=15
//! pol: preamble=76 lines
//!   exc: shared=0, delta=76, outputs=1
//!   vxc: shared=76, delta=101, outputs=6
//!   fxc: shared=177, delta=324, outputs=21
//!   kxc: shared=501, delta=964, outputs=56
//!   lxc: shared=1465, delta=1716, outputs=126

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
