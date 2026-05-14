//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 476/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk476<F: Float>(t2482: F, t888: F, t9263: F, t584: F, t6582: F) -> (F, F, F) {
    let t9264 = t888 * t2482;
    let t9265 = t9263 * t9264;
    let t9266 = 0.76685851907841499352e0 * t9265;
    let t9267 = t584 * t6582;
    (t9265, t9266, t9267)
}
