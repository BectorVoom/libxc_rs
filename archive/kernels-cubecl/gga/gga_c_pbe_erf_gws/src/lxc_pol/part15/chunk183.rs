//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 183/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk183<F: Float>(t481: F, t506: F, t127: F, t488: F, t491: F, t495: F, t496: F, t498: F, t504: F) -> F {
    let t507 = t506 * t481;
    let t510 = -t488 - t491 - t495 - t496 * t498 / F::cast_from(2.0_f64) - t504 - F::cast_from(0.146904e1_f64) * t127 * t507;
    t510
}
