//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1149/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1149(t2960: f64, t439: f64, t477: f64, t7481: f64, t19791: f64, t5260: f64, t1901: f64, t19754: f64, t2010: f64, t2002: f64, t6376: f64, t6379: f64) -> (f64, f64, f64, f64, f64) {
    let t20810 = 2.0_f64 / 9.0_f64 * t439 * t2960 * t7481 * t477;
    let t20813 = 32.0_f64 / 27.0_f64 * t439 * t5260 * t19791;
    let t20816 = 4.0_f64 / 3.0_f64 * t2010 * t1901 * t19754;
    let t20818 = 2.0_f64 / 15.0_f64 * t2002 * t6376;
    let t20820 = 2.0_f64 / 5.0_f64 * t2002 * t6379;
    (t20810, t20813, t20816, t20818, t20820)
}
