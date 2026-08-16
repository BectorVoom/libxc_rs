//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1114/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1114(t16749: f64, t1995: f64, t6134: f64, t165: f64, t1994: f64, t2553: f64, t493: f64, t136: f64, t1968: f64, t2582: f64, t439: f64, t529: f64, t7621: f64) -> (f64, f64, f64, f64, f64) {
    let t20380 = 4.0_f64 / 135.0_f64 * t16749;
    let t20382 = t6134 * t1995 / 5.0_f64;
    let t20386 = t493 * t165 * t2553 * t1994 / 5.0_f64;
    let t20390 = t439 * t136 * t2582 * t1968 / 5.0_f64;
    let t20391 = t7621 * t529;
    (t20380, t20382, t20386, t20390, t20391)
}
