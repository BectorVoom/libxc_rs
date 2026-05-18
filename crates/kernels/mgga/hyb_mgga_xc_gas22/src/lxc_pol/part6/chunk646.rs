//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 646/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk646<F: Float>(t1231: F, t668: F, t26: F, t215: F, t2950: F, t13: F, t2023: F, t2969: F) -> (F, F, F, F) {
    let t3118 = t1231 * t668;
    let t3119 = t26 * t3118;
    let t3124 = t2950 * t215;
    let t3138 = t2023 * t13 * t2969;
    (t3118, t3119, t3124, t3138)
}
