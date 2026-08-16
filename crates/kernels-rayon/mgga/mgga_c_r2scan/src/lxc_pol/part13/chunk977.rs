//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 977/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk977(t11166: f64, t374: f64, t3461: f64, t860: f64, t1100: f64, t2337: f64, t2867: f64, t481: f64, t3263: f64, t3262: f64, t3617: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11167 = t11166 * t374;
    let t11168 = t860 * t3461;
    let t11169 = 2.0_f64 * t11168;
    let t11170 = t1100 * t2337;
    let t11475 = t2867 * t481;
    let t11476 = t3263 * t11475;
    let t11477 = t3262 * t11476;
    let t11478 = 3.0_f64 / 4.0_f64 * t11477;
    let t11479 = t498 * t3617;
    (t11167, t11169, t11170, t11475, t11476, t11477, t11478, t11479)
}
