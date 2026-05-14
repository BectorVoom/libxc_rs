//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 958/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk958<F: Float>(t9112: F, t9115: F, t6969: F, t6972: F, t9119: F, t9123: F, t9127: F, t9136: F, t9138: F, t9140: F, t9143: F, t9145: F, t9011: F, t7021: F, t7035: F, t7037: F, t7040: F, t7043: F, t9008: F, t9029: F, t9149: F, t9152: F, t9155: F, t9159: F) -> (F, F, F, F, F) {
    let t9217 = 0.41678e0 * t9112;
    let t9218 = 0.41678e0 * t9115;
    let t9229 = -t9217 - t9218 + 0.312585e0 * t9119 + 0.62517e0 * t9123 + 0.312585e0 * t9127 + 0.13772666666666666667e1 * t6969 - 0.516475e0 * t6972 + 0.3529725e1 * t9136 + 0.6311625e0 * t9138 - 0.17648625e1 * t9140 + 0.6311625e0 * t9143 + 0.31558125e0 * t9145;
    let t9235 = 0.103295e1 * t9011;
    let t9240 = -0.3529725e1 * t9149 + 0.264729375e1 * t9152 - 0.157790625e0 * t9155 + 0.68863333333333333333e0 * t9008 + 0.34731666666666666667e0 * t9159 - t9235 + 0.1549425e1 * t9029 - t7021 - t7035 + 0.69463333333333333333e0 * t7037 - 0.20839e0 * t7040 - 0.20839e0 * t7043;
    (t9217, t9218, t9229, t9235, t9240)
}
