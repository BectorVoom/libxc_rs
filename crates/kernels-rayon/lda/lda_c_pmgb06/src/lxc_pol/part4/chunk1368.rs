//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1368/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1368(t17960: f64, t14211: f64, t14213: f64, t1600: f64, t6904: f64, t1992: f64, t493: f64, t529: f64, t5179: f64, t6113: f64, t1420: f64, t6255: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17961 = 2.0_f64 / 45.0_f64 * t17960;
    let t17962 = 4.0_f64 / 135.0_f64 * t14211;
    let t17963 = 4.0_f64 / 45.0_f64 * t14213;
    let t17964 = t1600 * t6904;
    let t17968 = 2.0_f64 / 15.0_f64 * t493 * t1992 * t17964 * t529;
    let t17971 = 2.0_f64 / 15.0_f64 * t493 * t5179 * t6113;
    let t17973 = 2.0_f64 / 5.0_f64 * t1420 * t6255;
    (t17961, t17962, t17963, t17968, t17971, t17973)
}
