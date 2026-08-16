//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 635/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk635(t177: f64, t3004: f64, t161: f64, t1423: f64, t1560: f64, t1166: f64, t539: f64, t188: f64, t1830: f64, t2060: f64, t83: f64, t1409: f64, t398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3005 = t3004 * t177;
    let t3007 = 4.0_f64 / 405.0_f64 * t161 * t3005;
    let t3008 = t1423 * t1560;
    let t3018 = t1166 * t539;
    let t3019 = t3018 * t188;
    let t3023 = 1.2833333333333334_f64 * t1830 - 20.0_f64 / 27.0_f64 * t2060;
    let t3024 = t83 * t3023;
    let t3026 = 4.0_f64 / 3.0_f64 * t3024 * t188;
    let t3027 = t398 * t1409;
    (t3005, t3007, t3008, t3018, t3019, t3023, t3024, t3026, t3027)
}
