//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 580/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk580<F: Float>(t2742: F, t462: F, t1057: F, t1068: F, t2696: F, t495: F) -> (F, F, F) {
    let t2743 = t462 * t2742;
    let t2745 = t1057 * t1068;
    let t2747 = t2696 * t495;
    (t2743, t2745, t2747)
}
