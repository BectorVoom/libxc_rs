//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1084/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1084<F: Float>(t1139: F, t4530: F, t4540: F, t2874: F, t4544: F, t1128: F, t1129: F, t3951: F, t647: F, t3748: F, t1160: F, t1535: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11447 = t1139 * t4530;
    let t11454 = t1139 * t4540;
    let t11461 = t2874 * t4544;
    let t11465 = t1128 * t4540;
    let t11466 = t11465 * t1129;
    let t11469 = t3951 * sigma0;
    let t11470 = t11469 * t647;
    let t11471 = t3748 * t11470;
    let t11474 = t1535 * t1160;
    (t11447, t11454, t11461, t11465, t11466, t11469, t11470, t11471, t11474)
}
