//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 781/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk781(t12: f64, t2200: f64, t2389: f64, t336: f64, t3548: f64, t7295: f64, t7300: f64, t7294: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t7304 = piecewise3(t13, 0.0_f64, 8.0_f64 / 27.0_f64 * t3548 * t7295 - 2.0_f64 / 3.0_f64 * t2200 * t2389 + 2.0_f64 / 3.0_f64 * t336 * t7300);
    let t7306 = t7294 / 2.0_f64 + t7304 / 2.0_f64;
    t7306
}
