//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 531/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk531<F: Float>(t3141: F, t501: F, t3145: F, t605: F, t2497: F, t921: F, t3207: F, t584: F, t6575: F) -> (F, F, F, F, F) {
    let t9243 = t3141 * t501;
    let t9253 = t3145 * t605;
    let t9256 = t921 * t2497;
    let t9260 = t3207 * t605;
    let t9263 = t584 * t6575;
    (t9243, t9253, t9256, t9260, t9263)
}
