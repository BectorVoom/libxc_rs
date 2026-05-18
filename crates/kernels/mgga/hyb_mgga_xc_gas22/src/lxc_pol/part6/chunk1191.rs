//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1191/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1191<F: Float>(t1068: F, t5891: F, t1067: F, t462: F, t7482: F, t21837: F, t2731: F, t7441: F, t1037: F, t1046: F, t2728: F, t7453: F) -> (F, F, F, F) {
    let t22061 = t5891 * t1068;
    let t22064 = t462 * t1067 * t7482;
    let t22068 = F::new(0.57895126195293126241e3) * t7441 * t21837 * t2731;
    let t22072 = F::new(0.34367190188705947437e1) * t1037 * t2728 * t1046 * t7453;
    (t22061, t22064, t22068, t22072)
}
