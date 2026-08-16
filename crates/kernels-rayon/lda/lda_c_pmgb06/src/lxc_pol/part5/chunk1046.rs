//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1046/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1046(t1444: f64, t7663: f64, t441: f64, t7501: f64, t439: f64, t445: f64, t224: f64, t7464: f64, t446: f64, t15807: f64, t1447: f64, t7509: f64) -> (f64, f64, f64, f64, f64) {
    let t19514 = t1444 * t7663 / 15.0_f64;
    let t19515 = t441 * t7501;
    let t19518 = t439 * t19515 * t445 / 45.0_f64;
    let t19519 = t7464 * t224;
    let t19521 = t19519 * t446 / 45.0_f64;
    let t19522 = t15807 / 15.0_f64;
    let t19523 = t1447 * t7509;
    (t19514, t19518, t19521, t19522, t19523)
}
