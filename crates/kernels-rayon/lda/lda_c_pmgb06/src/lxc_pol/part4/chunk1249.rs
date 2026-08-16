//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1249/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1249(t9596: f64, t9598: f64, t9601: f64, t490: f64, t6688: f64, t1504: f64, t2563: f64, t1366: f64, t7193: f64, t5102: f64, t831: f64, t161: f64, t166: f64, t2623: f64, t2885: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16439 = 8.0_f64 / 405.0_f64 * t9596;
    let t16440 = 2.0_f64 / 135.0_f64 * t9598;
    let t16441 = 2.0_f64 / 135.0_f64 * t9601;
    let t16442 = t6688 * t490;
    let t16443 = 2.0_f64 / 45.0_f64 * t16442;
    let t16444 = t2563 * t1504;
    let t16445 = 2.0_f64 / 45.0_f64 * t16444;
    let t16446 = t7193 * t1366;
    let t16448 = t831 * t5102;
    let t16449 = 4.0_f64 / 45.0_f64 * t16448;
    let t16453 = t161 * t166 * t2885 * t2623 / 30.0_f64;
    (t16439, t16440, t16441, t16443, t16445, t16446, t16449, t16453)
}
