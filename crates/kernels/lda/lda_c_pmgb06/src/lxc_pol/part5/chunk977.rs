//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 977/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk977<F: Float>(t1972: F, t6287: F, t1992: F, t2088: F, t493: F, t6112: F, t1444: F, t7685: F, t5179: F, t7684: F, t6119: F, t6527: F, t1420: F, t7690: F, t18016: F, t439: F, t805: F) -> (F, F, F, F, F, F, F) {
    let t20355 = 3.0 / 5.0 * t1972 * t6287;
    let t20359 = t493 * t1992 * t6112 * t2088 / 5.0;
    let t20361 = t1444 * t7685 / 5.0;
    let t20364 = t493 * t5179 * t7684 / 5.0;
    let t20367 = 2.0 / 5.0 * t493 * t6119 * t6527;
    let t20369 = t1420 * t7690 / 15.0;
    let t20372 = t439 * t18016 * t805 / 15.0;
    (t20355, t20359, t20361, t20364, t20367, t20369, t20372)
}
