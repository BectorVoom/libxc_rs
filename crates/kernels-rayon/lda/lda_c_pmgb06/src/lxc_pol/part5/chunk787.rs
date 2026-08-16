//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 787/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk787(t5: f64, t12: f64, t2125: f64, t2381: f64, t3912: f64, t7284: f64, t7290: f64, t9: f64, t14: f64, t2133: f64, t2389: f64, t3922: f64, t7295: f64, t7300: f64, zeta_threshold: f64) -> (f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t7392 = piecewise3(t6, 0.0_f64, -8.0_f64 / 27.0_f64 * t3912 * t7284 + 4.0_f64 / 3.0_f64 * t2125 * t2381 + 4.0_f64 / 3.0_f64 * t9 * t7290);
    let t7400 = piecewise3(t13, 0.0_f64, -8.0_f64 / 27.0_f64 * t3922 * t7295 + 4.0_f64 / 3.0_f64 * t2133 * t2389 + 4.0_f64 / 3.0_f64 * t14 * t7300);
    (t7392, t7400)
}
