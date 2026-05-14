//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 634/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk634<F: Float>(t1672: F, t611: F, t185: F, t108: F, t615: F, t267: F) -> (F, F, F, F) {
    let t5207 = t1672 * t611;
    let t5208 = t185 * t5207;
    let t5209 = 4.0 / 45.0 * t5208;
    let t5210 = t615 * t108;
    let t5211 = t5210 * t267;
    (t5207, t5209, t5210, t5211)
}
