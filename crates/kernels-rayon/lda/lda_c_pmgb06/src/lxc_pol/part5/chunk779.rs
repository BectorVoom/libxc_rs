//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 779/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk779(t5: f64, t2192: f64, t2381: f64, t330: f64, t3537: f64, t7284: f64, t7290: f64, t2386: f64, t764: f64, zeta_threshold: f64) -> (f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t7294 = piecewise3(t6, 0.0_f64, 8.0_f64 / 27.0_f64 * t3537 * t7284 - 2.0_f64 / 3.0_f64 * t2192 * t2381 + 2.0_f64 / 3.0_f64 * t330 * t7290);
    let t7295 = t2386 * t764;
    (t7294, t7295)
}
