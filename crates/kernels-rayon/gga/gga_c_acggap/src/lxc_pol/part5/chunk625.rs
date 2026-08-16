//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 625/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk625(t291: f64, t4099: f64, t96: f64, t119: f64, t1603: f64, t1308: f64, t872: f64, t1221: f64, t3875: f64, t556: f64, t1620: f64, t857: f64) -> (f64, f64, f64, f64, f64) {
    let t4101 = t96 * t291 * t4099;
    let t4103 = t119 * t1603;
    let t4107 = 0.13170898365871023197e1_f64 * t1308 * t872;
    let t4109 = t3875 * t556 * t1221;
    let t4113 = 0.26341796731742046394e1_f64 * t857 * t1620;
    (t4101, t4103, t4107, t4109, t4113)
}
