//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 989/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk989(t1413: f64, t3765: f64, t1181: f64, t3391: f64, t3754: f64, t540: f64, t13092: f64, t4732: f64, t3491: f64, t4282: f64, t535: f64, t4912: f64) -> (f64, f64, f64, f64, f64) {
    let t16398 = t3765 * t1413;
    let t16407 = t3391 * t1181 * t540 * t3754;
    let t16409 = t13092 * t4732;
    let t16415 = t4282 * t1181 * t535 * t3491;
    let t16417 = t13092 * t4912;
    (t16398, t16407, t16409, t16415, t16417)
}
