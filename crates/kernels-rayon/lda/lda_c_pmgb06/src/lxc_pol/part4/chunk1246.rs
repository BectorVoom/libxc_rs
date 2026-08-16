//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1246/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1246(t12772: f64, t1907: f64, t439: f64, t12603: f64, t12621: f64, t12623: f64, t12625: f64, t12631: f64, t13100: f64, t493: f64, t834: f64, t2462: f64, t3198: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16411 = 4.0_f64 / 45.0_f64 * t439 * t12772 * t1907;
    let t16412 = 8.0_f64 / 135.0_f64 * t12603;
    let t16413 = 8.0_f64 / 405.0_f64 * t12621;
    let t16414 = 8.0_f64 / 135.0_f64 * t12623;
    let t16415 = 8.0_f64 / 135.0_f64 * t12625;
    let t16416 = 8.0_f64 / 135.0_f64 * t12631;
    let t16419 = 2.0_f64 / 45.0_f64 * t493 * t13100 * t834;
    let t16421 = 2.0_f64 / 45.0_f64 * t3198 * t2462;
    (t16411, t16412, t16413, t16414, t16415, t16416, t16419, t16421)
}
