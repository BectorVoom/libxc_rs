//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1124/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1124<F: Float>(t1110: F, t21846: F, t2639: F, t7237: F, t2643: F, t7249: F, t21833: F, t2729: F, t2731: F, t1068: F, t7539: F, t2754: F, t2814: F, t2751: F, t221: F, t2631: F, t2696: F) -> (F, F, F, F, F, F, F) {
    let t22089 = 0.6233709278045326953e3 * t1110 * t7237 * t21846 * t2639;
    let t22090 = t2643 * t7249;
    let t22094 = 0.48245938496077605201e2 * t2729 * t21833 * t2731;
    let t22095 = t7539 * t1068;
    let t22102 = t2754 * t2814;
    let t22105 = 120.0 * t2751 * t2814;
    let t22107 = t2696 * t221 * t2631;
    (t22089, t22090, t22094, t22095, t22102, t22105, t22107)
}
