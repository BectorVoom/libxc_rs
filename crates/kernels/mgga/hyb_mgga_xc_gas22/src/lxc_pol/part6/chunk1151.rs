//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1151/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1151<F: Float>(t11310: F, t11357: F, t11399: F, t11444: F, t11495: F, t11530: F, t11573: F, t11614: F, t500: F, t1123: F, t4851: F, t1129: F) -> (F, F, F, F) {
    let t11617 = t11310 + t11357 + t11399 + t11444 + t11495 + t11530 + t11573 + t11614;
    let t11618 = t500 * t11617;
    let t13638 = t4851 * t1123;
    let t13643 = t4851 * t1129;
    (t11617, t11618, t13638, t13643)
}
