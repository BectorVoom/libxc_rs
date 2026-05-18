//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 638/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk638<F: Float>(t1193: F, t54: F, t3029: F, t588: F, t57: F, t592: F, t60: F, t596: F, t63: F, t600: F, t66: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3032 = t54 * t1193;
    let t3035 = t588 * t3029;
    let t3037 = t57 * t1193;
    let t3040 = t592 * t3029;
    let t3042 = t60 * t1193;
    let t3045 = t596 * t3029;
    let t3047 = t63 * t1193;
    let t3050 = t600 * t3029;
    let t3052 = t66 * t1193;
    let t3055 = t604 * t3029;
    (t3032, t3035, t3037, t3040, t3042, t3045, t3047, t3050, t3052, t3055)
}
