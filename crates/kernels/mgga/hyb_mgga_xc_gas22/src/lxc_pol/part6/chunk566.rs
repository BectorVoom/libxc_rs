//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 566/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk566<F: Float>(t1047: F, t2713: F, t2712: F, t2657: F, t2660: F, t2663: F, t2667: F, t2669: F, t2672: F) -> (F, F, F) {
    let t2714 = t2713 * t1047;
    let t2716 = 2.0 * t2712 * t2714;
    let t2723 = -0.42198333333333333333e0 * t2657 + 0.84396666666666666666e0 * t2660 + 0.39862222222222222223e0 * t2663 + 0.68258333333333333333e-1 * t2667 + 0.13651666666666666667e0 * t2669 + 0.13692777777777777778e0 * t2672;
    (t2714, t2716, t2723)
}
