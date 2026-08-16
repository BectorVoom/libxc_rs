//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1147/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1147(t1972: f64, t6292: f64, t1444: f64, t7656: f64, t2488: f64, t493: f64, t5312: f64, t1420: f64, t7646: f64, t17577: f64, t432: f64, t7803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20784 = 2.0_f64 / 15.0_f64 * t1972 * t6292;
    let t20786 = 2.0_f64 / 15.0_f64 * t1444 * t7656;
    let t20789 = 2.0_f64 / 15.0_f64 * t493 * t5312 * t2488;
    let t20791 = 2.0_f64 / 9.0_f64 * t1420 * t7646;
    let t20792 = 4.0_f64 / 45.0_f64 * t17577;
    let t20794 = t432 * t7803 / 30.0_f64;
    (t20784, t20786, t20789, t20791, t20792, t20794)
}
