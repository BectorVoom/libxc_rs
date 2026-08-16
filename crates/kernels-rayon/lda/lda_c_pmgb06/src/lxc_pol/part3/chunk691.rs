//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 691/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk691(t12: f64, t3548: f64, t764: f64, t1: f64, t1219: f64, t337: f64, t395: f64, t1080: f64, t1083: f64, t2200: f64, t2203: f64, t247: f64, t336: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t4378 = t3548 * t764;
    let t4381 = t1219 * t1;
    let t4382 = t337 * t395;
    let t4392 = piecewise3(t13, 0.0_f64, 8.0_f64 / 27.0_f64 * t4378 * t1080 + 8.0_f64 / 9.0_f64 * t4381 * t4382 - 2.0_f64 / 9.0_f64 * t2200 * t1083 - 4.0_f64 / 3.0_f64 * t336 * t395 + 4.0_f64 * t2203 * t247);
    (t4378, t4381, t4382, t4392)
}
