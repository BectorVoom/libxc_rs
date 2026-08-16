//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 504/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk504(t12: f64, t176: f64, t2553: f64, t166: f64, t161: f64, t2386: f64, t2389: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t2554 = t2553 * t176;
    let t2555 = t166 * t2554;
    let t2557 = t161 * t2555 / 30.0_f64;
    let t2561 = piecewise3(t13, 0.0_f64, 2.0_f64 * t12 * t2389 + 2.0_f64 * t2386);
    let t2562 = t2561 * t44;
    let t2563 = t2562 * t131;
    (t2554, t2555, t2557, t2562, t2563)
}
