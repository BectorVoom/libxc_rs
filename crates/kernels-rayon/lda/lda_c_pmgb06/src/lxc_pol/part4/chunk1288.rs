//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1288/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1288(t16925: f64, t1423: f64, t6124: f64, t439: f64, t5197: f64, t6555: f64, t1512: f64, t2631: f64, t432: f64, t6600: f64, t1392: f64, t2592: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16926 = 4.0_f64 / 135.0_f64 * t16925;
    let t16927 = t1423 * t6124;
    let t16928 = 4.0_f64 / 135.0_f64 * t16927;
    let t16931 = 2.0_f64 / 15.0_f64 * t439 * t5197 * t6555;
    let t16933 = t1512 * t2631 / 15.0_f64;
    let t16935 = 2.0_f64 / 15.0_f64 * t432 * t6600;
    let t16936 = t2592 * t1392;
    (t16926, t16928, t16931, t16933, t16935, t16936)
}
