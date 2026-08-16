//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 533/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk533<F: Float>(t3096: F, t619: F, t3094: F, t1026: F, t628: F, t205: F, t126: F, t95: F) -> (F, F, F, F, F) {
    let t3097 = t3096 * t619;
    let t3098 = t3094 * t3097;
    let t3100 = t628 * t1026;
    let t3101 = t3100 * t205;
    let t3103 = t126 * t95;
    (t3097, t3098, t3100, t3101, t3103)
}
