//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 900/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk900(t1029: f64, t3228: f64, t166: f64, t1: f64, t1035: f64, t1039: f64, t3669: f64, t3036: f64, t3213: f64, t996: f64, t117: f64, t3033: f64) -> (f64, f64, f64, f64, f64) {
    let t13459 = t3228 * t1029;
    let t13461 = t166 * t166;
    let t13462 = 1.0_f64 / t13461;
    let t13463 = t13462 * t1;
    let t13474 = 0.68026775414003982664e-1_f64 * t1035 * t3669 * t1039;
    let t13481 = 0.24009450146119052705e-1_f64 * t3036 * t996 * t3213;
    let t13483 = 1.0_f64 / t3033 / t117;
    (t13459, t13463, t13474, t13481, t13483)
}
