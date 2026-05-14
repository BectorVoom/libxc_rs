//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 821/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk821<F: Float>(t1868: F, t822: F, t1385: F, t2010: F, t2574: F, t477: F, t439: F, t1897: F, t6185: F, t6189: F, t1444: F, t2497: F, t2496: F, t2979: F, t493: F, t2088: F, t838: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6371 = t1868 * t822;
    let t6372 = t1385 * t6371;
    let t6374 = 4.0 / 45.0 * t2010 * t6372;
    let t6375 = t2574 * t477;
    let t6376 = t1385 * t6375;
    let t6378 = 2.0 / 45.0 * t439 * t6376;
    let t6379 = t1897 * t6185;
    let t6381 = 2.0 / 15.0 * t439 * t6379;
    let t6382 = t1897 * t6189;
    let t6384 = 8.0 / 45.0 * t2010 * t6382;
    let t6386 = 2.0 / 45.0 * t1444 * t2497;
    let t6387 = t2979 * t2496;
    let t6389 = 2.0 / 45.0 * t493 * t6387;
    let t6390 = t838 * t2088;
    (t6371, t6372, t6374, t6375, t6376, t6378, t6379, t6381, t6382, t6384, t6386, t6387, t6389, t6390)
}
