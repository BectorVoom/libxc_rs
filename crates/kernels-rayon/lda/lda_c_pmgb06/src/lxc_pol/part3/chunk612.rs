//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 612/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk612(t3427: f64, t3440: f64, t465: f64, t137: f64, t132: f64, t1586: f64, t1639: f64, t166: f64, t161: f64, t1554: f64, t530: f64, t1587: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3441 = t3427 + t3440;
    let t3442 = t465 * t3441;
    let t3443 = t137 * t3442;
    let t3445 = t132 * t3443 / 30.0_f64;
    let t3446 = t1639 * t1586;
    let t3447 = t166 * t3446;
    let t3449 = t161 * t3447 / 10.0_f64;
    let t3450 = t1554 * t530;
    let t3451 = t161 * t3450;
    let t3452 = t3451 / 45.0_f64;
    let t3453 = t489 * t1587;
    (t3441, t3442, t3443, t3445, t3446, t3447, t3449, t3450, t3451, t3452, t3453)
}
