//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1152/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1152<F: Float>(t1129: F, t1297: F, t1117: F, t1128: F, t2880: F, t510: F, t2903: F, t521: F, t1134: F, t1139: F, t2874: F, t518: F) -> (F, F, F, F, F, F) {
    let t13687 = t1297 * t1129;
    let t14626 = t1117 * t1128;
    let t14635 = t510 * t2880;
    let t14638 = t2903 * t521;
    let t14641 = t1134 * t1139;
    let t14648 = t518 * t2874;
    (t13687, t14626, t14635, t14638, t14641, t14648)
}
