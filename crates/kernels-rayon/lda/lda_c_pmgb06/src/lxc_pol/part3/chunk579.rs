//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 579/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk579(t1525: f64, t3104: f64, t36: f64, t1438: f64, t3010: f64, t453: f64, t2970: f64, t2938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3105 = t1525 * t3104;
    let t3106 = t36 * t3105;
    let t3108 = t1438 * t3010;
    let t3109 = t453 * t3108;
    let t3110 = t36 * t3109;
    let t3112 = t453 * t2970;
    let t3113 = t36 * t3112;
    let t3115 = -t2938;
    (t3105, t3106, t3108, t3109, t3110, t3112, t3113, t3115)
}
