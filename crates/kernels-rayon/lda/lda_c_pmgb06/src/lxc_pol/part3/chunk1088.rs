//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1088/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1088(t12: f64, t1952: f64, t642: f64, t11039: f64, t11047: f64, t11050: f64, t15: f64, t1949: f64, t2200: f64, t2203: f64, t247: f64, t2912: f64, t2938: f64, t3139: f64, t337: f64, t395: f64, t4700: f64, t598: f64, t765: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t12960 = 64.0_f64 * t1952 * t642;
    let t12962 = piecewise3(t13, 0.0_f64, -80.0_f64 / 81.0_f64 * t2200 * t2912 - 160.0_f64 / 9.0_f64 * t2203 * t11039 + 80.0_f64 / 9.0_f64 * t765 * t3139 - 80.0_f64 / 3.0_f64 * t15 * t395 * t337 + 80.0_f64 * t4700 * t11047 - 80.0_f64 / 3.0_f64 * t4700 * t11050 + 40.0_f64 / 9.0_f64 * t1949 * t2938 + 32.0_f64 * t598 * t247 - t12960);
    t12962
}
