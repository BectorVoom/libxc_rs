//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 786/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk786(t12: f64, t176: f64, t5415: f64, t166: f64, t161: f64, t1: f64, t337: f64, t395: f64, t1083: f64, t1842: f64, t247: f64, t764: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t5416 = t5415 * t176;
    let t5417 = t166 * t5416;
    let t5419 = t161 * t5417 / 30.0_f64;
    let t5422 = t337 * t1;
    let t5423 = t5422 * t395;
    let t5430 = piecewise3(t13, 0.0_f64, 2.0_f64 * t1083 * t764 - 4.0_f64 * t12 * t395 + 12.0_f64 * t1842 * t247 - 8.0_f64 * t5423);
    (t5416, t5417, t5419, t5422, t5430)
}
