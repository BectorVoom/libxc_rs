//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1092/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1092<F: Float>(t34418: F, t47376: F, t47381: F, t47385: F, t47387: F, t47389: F, t47506: F, t47507: F, t47511: F, t47515: F, t47519: F, t47523: F) -> F {
    let t47524 = t47376 + t47381 - t47385 - t47387 + F::cast_from(2.0_f64) * t34418 - t47389 - t47506 - t47507 - t47511 - t47515 + t47519 - t47523;
    t47524
}
