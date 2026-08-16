//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 958/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk958(t12: f64, t2203: f64, t642: f64, t1: f64, t11039: f64, t11047: f64, t11050: f64, t1219: f64, t2200: f64, t247: f64, t2912: f64, t2938: f64, t3139: f64, t336: f64, t337: f64, t3548: f64, t395: f64, t4378: f64, t4381: f64, t764: f64, t8139: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t11282 = 16.0_f64 * t2203 * t642;
    let t11284 = piecewise3(t13, 0.0_f64, -56.0_f64 / 81.0_f64 * t8139 * t764 * t2912 - 16.0_f64 / 9.0_f64 * t3548 * t1 * t11039 + 8.0_f64 / 9.0_f64 * t4378 * t3139 + 4.0_f64 / 3.0_f64 * t1219 * t395 * t337 - 4.0_f64 * t4381 * t11047 + 4.0_f64 / 3.0_f64 * t4381 * t11050 - 2.0_f64 / 9.0_f64 * t2200 * t2938 + 8.0_f64 * t336 * t247 - t11282);
    t11284
}
