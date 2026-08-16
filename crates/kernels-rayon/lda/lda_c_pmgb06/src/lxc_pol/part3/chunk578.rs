//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 578/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk578(t3010: f64, t3098: f64, t1525: f64, t36: f64, t1438: f64, t332: f64, t1074: f64) -> (f64, f64, f64, f64, f64) {
    let t3099 = t3098 * t3010;
    let t3100 = t1525 * t3099;
    let t3101 = t36 * t3100;
    let t3103 = t1438 * t332;
    let t3104 = t3103 * t1074;
    (t3099, t3100, t3101, t3103, t3104)
}
