//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 943/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk943<F: Float>(t145: F, t169: F, t242: F, t5700: F, t5723: F, t5726: F, t5730: F, t5732: F, t5735: F, t8038: F, t8363: F, t8365: F, t8371: F, t8373: F) -> F {
    let t8379 = -F::cast_from(0.1066501354843587606e0_f64) * t5735 - F::cast_from(0.14149184788746388121e0_f64) * t8363 - F::cast_from(0.31835665774679373271e-1_f64) * t169 * t8365 * t242 - t8371 - F::cast_from(0.31835665774679373271e-1_f64) * t8373 + F::cast_from(0.533250677421793803e-1_f64) * t145 * t8038 - F::cast_from(0.31835665774679373271e-1_f64) * t5723 - F::cast_from(0.63671331549358746542e-1_f64) * t5726 - t5730 + t5700 - t5732;
    t8379
}
