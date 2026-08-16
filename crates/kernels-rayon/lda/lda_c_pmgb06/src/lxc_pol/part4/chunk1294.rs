//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1294/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1294(t1969: f64, t5220: f64, t1981: f64, t1982: f64, t5312: f64, t4602: f64, t6536: f64, t1444: f64, t6282: f64, t10216: f64, t2469: f64, t493: f64) -> (f64, f64, f64, f64, f64) {
    let t16992 = t5220 * t1969;
    let t16993 = 8.0_f64 / 45.0_f64 * t16992;
    let t16996 = 8.0_f64 / 45.0_f64 * t1981 * t5312 * t1982;
    let t16998 = 8.0_f64 / 45.0_f64 * t4602 * t6536;
    let t17000 = 2.0_f64 / 27.0_f64 * t1444 * t6282;
    let t17003 = t493 * t10216 * t2469 / 27.0_f64;
    (t16993, t16996, t16998, t17000, t17003)
}
