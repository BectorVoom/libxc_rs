//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 478/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk478(t12: f64, t1079: f64, t14: f64, t2386: f64, t2389: f64, t2385: f64, t257: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t2393 = piecewise3(t13, 0.0_f64, 4.0_f64 / 9.0_f64 * t1079 * t2386 + 4.0_f64 / 3.0_f64 * t14 * t2389);
    let t2395 = (t2385 + t2393) * t257;
    t2395
}
