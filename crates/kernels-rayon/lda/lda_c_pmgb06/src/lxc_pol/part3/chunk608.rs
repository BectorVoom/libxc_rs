//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 608/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk608(t3382: f64, t518: f64, t166: f64, t161: f64, t1400: f64, t187: f64, t186: f64, t395: f64, t184: f64, t1403: f64, t1410: f64, t474: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3383 = t518 * t3382;
    let t3384 = t166 * t3383;
    let t3386 = t161 * t3384 / 30.0_f64;
    let t3387 = t1400 * t187;
    let t3389 = t395 * t186;
    let t3391 = 0.0011033703703703704_f64 * t184 * t3389;
    let t3392 = t1403 * t187;
    let t3395 = 4.0_f64 * t1410 * t187;
    let t3396 = t955 * t474;
    (t3383, t3384, t3386, t3387, t3389, t3391, t3392, t3395, t3396)
}
