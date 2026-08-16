//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1061/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1061(t1008: f64, t5014: f64, t1181: f64, t12936: f64, t3650: f64, t540: f64, t119: f64, t12331: f64, t150: f64, t1162: f64, t13287: f64, t13293: f64, t16507: f64, t525: f64) -> (f64, f64, f64, f64, f64) {
    let t18743 = t1008 * t5014;
    let t18747 = t12936 * t1181 * t540 * t3650;
    let t18750 = t119 * t150 * t12331;
    let t18751 = t18750 * t1162;
    let t18763 = t13293 * t13287 * t525 * t16507;
    (t18743, t18747, t18750, t18751, t18763)
}
