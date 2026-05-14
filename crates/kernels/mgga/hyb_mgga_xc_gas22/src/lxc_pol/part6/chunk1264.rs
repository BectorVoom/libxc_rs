//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1264/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1264<F: Float>(t25262: F, t8997: F, t2515: F, t2521: F, t4244: F, t2479: F, t4273: F, t7148: F, t1410: F, t2478: F, t9167: F, t25267: F, t3482: F, t4270: F, t4296: F, t7061: F) -> (F, F, F, F, F, F, F) {
    let t29660 = 0.1034520258385468006e4 * t25262 * t8997;
    let t29663 = 6.0 * t2521 * t4244 * t2515;
    let t29666 = 0.57895126195293126241e3 * t7148 * t4273 * t2479;
    let t29669 = 4.0 * t2478 * t1410 * t9167;
    let t29671 = 8.0 * t25267 * t3482;
    let t29674 = 6.0 * t2521 * t4270 * t2479;
    let t29684 = t4296 * t7061;
    (t29660, t29663, t29666, t29669, t29671, t29674, t29684)
}
