//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1289/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1289(t16936: f64, t479: f64, t6705: f64, t1397: f64, t2592: f64, t13192: f64, t13194: f64, t1444: f64, t6120: f64, t16916: f64, t16919: f64, t16921: f64, t16923: f64, t16926: f64, t16928: f64, t16931: f64, t16933: f64, t16935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16937 = 2.0_f64 / 45.0_f64 * t16936;
    let t16939 = t6705 * t479 / 15.0_f64;
    let t16941 = t2592 * t1397 / 15.0_f64;
    let t16942 = 4.0_f64 / 45.0_f64 * t13192;
    let t16943 = 4.0_f64 / 45.0_f64 * t13194;
    let t16945 = 4.0_f64 / 15.0_f64 * t1444 * t6120;
    let t16946 = -t16916 - t16919 + t16921 + t16923 + t16926 + t16928 + t16931 - t16933 - t16935 - t16937 - t16939 - t16941 - t16942 - t16943 + t16945;
    (t16937, t16939, t16941, t16942, t16943, t16945, t16946)
}
