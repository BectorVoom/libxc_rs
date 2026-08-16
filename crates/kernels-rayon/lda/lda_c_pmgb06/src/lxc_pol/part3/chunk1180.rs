//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1180/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1180(t146: f64, t4918: f64, t9712: f64, t1575: f64, t2918: f64, t10006: f64, t12547: f64, t13382: f64, t13386: f64, t13390: f64, t13394: f64, t13399: f64, t13402: f64, t13405: f64, t13565: f64, t9503: f64, t9505: f64, t9577: f64, t9960: f64, t9962: f64, t9974: f64, t9981: f64, t9986: f64, t9987: f64) -> f64 {
    let t14150 = t146 * t9712 * t4918;
    let t14152 = t1575 * t2918;
    let t14160 = 0.0044444444444444444_f64 * t9960 + 0.0019753086419753087_f64 * t9962 - 0.008888888888888889_f64 * t9974 - 0.5038833333333333_f64 * t13382 + t9981 - 0.11997222222222222_f64 * t13386 + 0.4319_f64 * t13390 - 0.64785_f64 * t13394 + 0.09597777777777777_f64 * t9577 + 0.023994444444444443_f64 * t9503 - 0.07198333333333333_f64 * t9505 + t9986 - 0.02666666666666667_f64 * t9987 - 0.10666666666666667_f64 * t14150 + 0.04_f64 * t13565 * t14152 * t12547 + 0.0044444444444444444_f64 * t10006 + 1.1757277777777777_f64 * t13399 + 0.14396666666666666_f64 * t13402 - 0.4319_f64 * t13405;
    t14160
}
