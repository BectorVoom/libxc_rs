//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1007/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1007(t1602: f64, t1831: f64, t1981: f64, t2871: f64, t153: f64, t1864: f64, t3216: f64, t439: f64, t1444: f64, t5333: f64, t4861: f64, t493: f64, t5447: f64) -> (f64, f64, f64, f64) {
    let t11981 = 4.0_f64 / 15.0_f64 * t1981 * t2871 * t1831 * t1602;
    let t11985 = 2.0_f64 / 15.0_f64 * t439 * t3216 * t153 * t1864;
    let t11987 = 2.0_f64 / 5.0_f64 * t1444 * t5333;
    let t11990 = 2.0_f64 / 5.0_f64 * t493 * t5447 * t4861;
    (t11981, t11985, t11987, t11990)
}
