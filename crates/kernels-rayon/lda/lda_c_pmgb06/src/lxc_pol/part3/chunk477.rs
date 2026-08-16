//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 477/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk477(t12: f64, t1: f64, t598: f64, t1949: f64, t337: f64, t395: f64, t1948: f64, t44: f64, t441: f64, t813: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t1952 = t598 * t1;
    let t1956 = piecewise3(t13, 0.0_f64, 40.0_f64 / 9.0_f64 * t1949 * t337 - 16.0_f64 / 3.0_f64 * t1952 * t395);
    let t1959 = (t1948 / 2.0_f64 + t1956 / 2.0_f64) * t44;
    let t1962 = t441 * t813;
    (t1952, t1959, t1962)
}
