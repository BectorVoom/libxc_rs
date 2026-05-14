//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 499/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk499<F: Float>(t1758: F, t3421: F, t11: F, t1764: F, t3342: F, t571: F, t3346: F, t572: F) -> (F, F, F, F, F, F) {
    let t3422 = t1758 * t3421;
    let t3423 = t11 * t3422;
    let t3425 = t1764 * t3342;
    let t3426 = t571 * t3425;
    let t3427 = t11 * t3426;
    let t3429 = t572 * t3346;
    (t3422, t3423, t3425, t3426, t3427, t3429)
}
