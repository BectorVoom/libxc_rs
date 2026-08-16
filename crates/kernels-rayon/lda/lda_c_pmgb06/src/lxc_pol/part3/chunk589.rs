//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 589/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk589(t3198: f64, t500: f64, t1417: f64, t1447: f64, t1465: f64, t2912: f64, t496: f64, t493: f64, t3164: f64, t3166: f64, t3168: f64, t3171: f64, t3176: f64, t3179: f64, t3181: f64, t3183: f64, t3185: f64, t3188: f64, t3193: f64, t3197: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3200 = t3198 * t500 / 15.0_f64;
    let t3201 = t1447 * t1417;
    let t3202 = 4.0_f64 / 45.0_f64 * t3201;
    let t3203 = t1465 * t2912;
    let t3204 = t496 * t3203;
    let t3206 = 2.0_f64 / 15.0_f64 * t493 * t3204;
    let t3207 = t3164 + t3166 - t3168 - t3171 - t3176 + t3179 + t3181 + t3183 - t3185 - t3188 - t3193 + t3197 + t3200 - t3202 + t3206;
    (t3200, t3201, t3202, t3203, t3204, t3206, t3207)
}
