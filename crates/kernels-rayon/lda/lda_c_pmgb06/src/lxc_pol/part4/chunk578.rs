//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 578/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk578(t2501: f64, t439: f64, t2464: f64, t2468: f64, t2472: f64, t2474: f64, t2476: f64, t2479: f64, t2483: f64, t2487: f64, t2491: f64, t2495: f64, t2499: f64) -> (f64, f64) {
    let t2503 = 2.0_f64 / 45.0_f64 * t439 * t2501;
    let t2504 = t2464 + t2468 + t2472 + t2474 + t2476 + t2479 + t2483 + t2487 - t2491 - t2495 - t2499 - t2503;
    (t2503, t2504)
}
