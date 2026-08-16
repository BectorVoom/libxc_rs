//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 742/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk742(t5005: f64, t5038: f64, t465: f64, t137: f64, t132: f64, t1554: f64, t843: f64, t161: f64, t1555: f64, t831: f64, t1548: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5039 = t5005 + t5038;
    let t5040 = t465 * t5039;
    let t5041 = t137 * t5040;
    let t5043 = t132 * t5041 / 30.0_f64;
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5046 = t5045 / 135.0_f64;
    let t5047 = t831 * t1555;
    let t5048 = t5047 / 135.0_f64;
    let t5049 = t802 * t1548;
    (t5039, t5040, t5041, t5043, t5044, t5046, t5048, t5049)
}
