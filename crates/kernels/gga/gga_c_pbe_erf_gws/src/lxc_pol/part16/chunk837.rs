//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 837/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk837<F: Float>(t7993: F, t8006: F, t8025: F, t8036: F, t242: F, t3013: F, t153: F, t2848: F, t542: F, t145: F, t2522: F, t2519: F, t700: F, t41: F, t7908: F, t2523: F) -> (F, F, F, F, F, F, F, F) {
    let t8038 = t7993 + t8006 + t8025 + t8036;
    let t8042 = t3013 * t242;
    let t8047 = 0.11389037339096724978e1 * t153 * t542 * t2848;
    let t8048 = t145 * t2522;
    let t8050 = 0.16752564107100880375e0 * t8048 * t242;
    let t8051 = t2519 * t700;
    let t8053 = t41 * t7908;
    let t8057 = 0.16752564107100880375e0 * t2523 * t700;
    (t8038, t8042, t8047, t8048, t8050, t8051, t8053, t8057)
}
