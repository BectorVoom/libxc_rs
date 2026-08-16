//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 495/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk495(t2493: f64, t439: f64, t838: f64, t851: f64, t1380: f64) -> (f64, f64, f64) {
    let t2495 = 2.0_f64 / 45.0_f64 * t439 * t2493;
    let t2496 = t838 * t851;
    let t2497 = t1380 * t2496;
    (t2495, t2496, t2497)
}
