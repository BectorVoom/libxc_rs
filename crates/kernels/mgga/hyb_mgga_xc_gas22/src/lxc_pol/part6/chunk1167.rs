//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1167/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1167<F: Float>(t1421: F, t7002: F, t3546: F, t7061: F, t2601: F, t9195: F, t3579: F, t6996: F, t7059: F, t2560: F, t1433: F, t2599: F, t7109: F, t6993: F, t1409: F, t2521: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25661 = t7002 * t1421;
    let t25680 = t3546 * t7061;
    let t25730 = t9195 * t2601;
    let t25737 = t3579 * t6996;
    let t25806 = t7059 * t1421;
    let t25810 = t2560 * t1421;
    let t25813 = t2599 * t1433;
    let t25816 = t7109 * t1433;
    let t25819 = t6993 * t1433;
    let t25823 = t2521 * t1409;
    (t25661, t25680, t25730, t25737, t25806, t25810, t25813, t25816, t25819, t25823)
}
