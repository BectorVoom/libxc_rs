//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 581/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk581(t5: f64, t153: f64, t3120: f64, t137: f64, t132: f64, t1542: f64, t432: f64, t1074: f64, t332: f64, t3115: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t3121 = t3120 * t153;
    let t3122 = t137 * t3121;
    let t3124 = t132 * t3122 / 30.0_f64;
    let t3126 = t432 * t1542 / 10.0_f64;
    let t3127 = t332 * t1074;
    let t3132 = piecewise3(t6, 0.0_f64, 2.0_f64 * t5 * t3115 + 6.0_f64 * t3127);
    let t3133 = t3132 * t44;
    let t3134 = t3133 * t131;
    (t3121, t3122, t3124, t3126, t3127, t3133, t3134)
}
