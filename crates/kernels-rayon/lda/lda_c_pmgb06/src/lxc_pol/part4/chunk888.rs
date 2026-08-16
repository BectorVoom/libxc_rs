//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 888/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk888(t1868: f64, t822: f64, t1385: f64, t2010: f64, t2574: f64, t477: f64, t439: f64, t1897: f64, t6185: f64, t6189: f64, t1444: f64, t2497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6371 = t1868 * t822;
    let t6372 = t1385 * t6371;
    let t6374 = 4.0_f64 / 45.0_f64 * t2010 * t6372;
    let t6375 = t2574 * t477;
    let t6376 = t1385 * t6375;
    let t6378 = 2.0_f64 / 45.0_f64 * t439 * t6376;
    let t6379 = t1897 * t6185;
    let t6381 = 2.0_f64 / 15.0_f64 * t439 * t6379;
    let t6382 = t1897 * t6189;
    let t6384 = 8.0_f64 / 45.0_f64 * t2010 * t6382;
    let t6386 = 2.0_f64 / 45.0_f64 * t1444 * t2497;
    (t6371, t6372, t6374, t6375, t6376, t6378, t6379, t6381, t6382, t6384, t6386)
}
