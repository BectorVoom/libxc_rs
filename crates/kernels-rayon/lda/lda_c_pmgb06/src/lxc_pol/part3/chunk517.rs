//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 517/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk517(t5: f64, t12: f64, t2192: f64, t2195: f64, t332: f64, t395: f64, t1219: f64, t764: f64, t1: f64, t336: f64, t337: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t2199 = piecewise3(t6, 0.0_f64, -2.0_f64 / 9.0_f64 * t2192 * t332 + 4.0_f64 / 3.0_f64 * t2195 * t395);
    let t2200 = t1219 * t764;
    let t2203 = t336 * t1;
    let t2207 = piecewise3(t13, 0.0_f64, -2.0_f64 / 9.0_f64 * t2200 * t337 - 4.0_f64 / 3.0_f64 * t2203 * t395);
    let t2209 = t2199 / 2.0_f64 + t2207 / 2.0_f64;
    (t2200, t2203, t2209)
}
