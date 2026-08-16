//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 711/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk711(t2604: f64, t443: f64, t332: f64, t2864: f64, t439: f64, t1993: f64, t2088: f64, t1992: f64, t493: f64, t1444: f64, t2462: f64, t5312: f64, t834: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6522 = t2604 * t443;
    let t6523 = t6522 * t332;
    let t6524 = t2864 * t6523;
    let t6526 = 2.0_f64 / 45.0_f64 * t439 * t6524;
    let t6527 = t1993 * t2088;
    let t6528 = t1992 * t6527;
    let t6530 = 2.0_f64 / 15.0_f64 * t493 * t6528;
    let t6532 = 2.0_f64 / 45.0_f64 * t1444 * t2462;
    let t6533 = t5312 * t834;
    (t6522, t6523, t6524, t6526, t6527, t6528, t6530, t6532, t6533)
}
