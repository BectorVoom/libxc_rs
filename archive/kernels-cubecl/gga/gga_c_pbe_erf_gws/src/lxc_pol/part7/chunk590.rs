//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 590/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk590<F: Float>(t2382: F, t4473: F, t833: F, t2222: F, t840: F, t814: F, t898: F, t938: F, t353: F, t859: F) -> (F, F, F, F, F) {
    let t4474 = t2382 * t4473;
    let t4475 = t4474 * t833;
    let t4477 = t840 * t2222;
    let t4482 = t898 * t814 * t938;
    let t4483 = t353 * t4482;
    let t4484 = t859 * t4483;
    (t4474, t4475, t4477, t4482, t4484)
}
