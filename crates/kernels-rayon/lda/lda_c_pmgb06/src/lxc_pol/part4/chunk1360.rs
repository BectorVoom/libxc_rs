//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1360/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1360(t17875: f64, t1631: f64, t2592: f64, t14011: f64, t14015: f64, t14465: f64, t14467: f64, t14469: f64, t14471: f64, t17858: f64, t17859: f64, t17861: f64, t17863: f64, t17869: f64, t17871: f64, t17873: f64) -> (f64, f64, f64, f64, f64) {
    let t17876 = 2.0_f64 / 45.0_f64 * t17875;
    let t17878 = t2592 * t1631 / 30.0_f64;
    let t17879 = 2.0_f64 / 45.0_f64 * t14011;
    let t17880 = 4.0_f64 / 135.0_f64 * t14015;
    let t17881 = -t17858 + 8.0_f64 / 3.0_f64 * t17859 + 8.0_f64 / 3.0_f64 * t17861 + t17863 + 8.0_f64 * t14465 + 0.002206740740740741_f64 * t14467 + 8.0_f64 / 3.0_f64 * t14469 + 32.0_f64 / 3.0_f64 * t14471 - t17869 - t17871 + t17873 - t17876 - t17878 - t17879 + t17880;
    (t17876, t17878, t17879, t17880, t17881)
}
