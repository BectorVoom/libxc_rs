//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1222/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1222<F: Float>(t684: F, t8477: F, t8485: F, t13: F, t20241: F, t2969: F, t3138: F, t3142: F, t763: F, t8497: F, t8498: F, t8506: F) -> (F, F, F, F) {
    let t23856 = t684 * t8477 * t8485;
    let t23872 = t20241 * t13 * t2969;
    let t23883 = t3138 * t8497 * t763 * t3142;
    let t23886 = t3138 * t8498 * t8506;
    (t23856, t23872, t23883, t23886)
}
