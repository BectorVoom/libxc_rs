//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 306/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk306<F: Float>(t475: F, t470: F, t1035: F, t1037: F, t1041: F, t1044: F) -> (F, F, F, F) {
    let t1073 = t475 * t475;
    let t1074 = 1.0 / t1073;
    let t1075 = t470 * t1074;
    let t1080 = -0.1176575e1 * t1035 - 0.516475e0 * t1037 - 0.2103875e0 * t1041 - 0.104195e0 * t1044;
    (t1073, t1074, t1075, t1080)
}
