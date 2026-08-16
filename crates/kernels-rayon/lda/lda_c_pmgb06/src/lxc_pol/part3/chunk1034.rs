//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1034/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1034(t12: f64, t1: f64, t1083: f64, t12294: f64, t247: f64, t2938: f64, t395: f64, t4382: f64, t5422: f64, t764: f64, t131: f64, t178: f64, t44: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t12296 = piecewise3(t13, 0.0_f64, -12.0_f64 * t1083 * t1 * t395 + 24.0_f64 * t12 * t247 + 36.0_f64 * t5422 * t247 + 2.0_f64 * t2938 * t764 - t12294 - 12.0_f64 * t4382);
    let t12300 = t12296 * t44 * t131 * t178 / 30.0_f64;
    t12300
}
