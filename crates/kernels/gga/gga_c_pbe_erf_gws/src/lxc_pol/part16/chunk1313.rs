//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1313/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1313<F: Float>(t53178: F, t53198: F, t53230: F, t14182: F, t14193: F, t22379: F, t2352: F, t2409: F, t26604: F, t3066: F, t3067: F, t4227: F, t52197: F, t52199: F, t53174: F, t53182: F, t53207: F, t53212: F, t53227: F, t53234: F, t53238: F) -> F {
    let t55005 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53178;
    let t55007 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53198;
    let t55022 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53230;
    let t55025 = t53174 / F::cast_from(384.0_f64) - t55005 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t53182 - t55007 + t3066 * t2409 * t3067 * t4227 * t2352 / F::cast_from(48.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t53207 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t52197 + t53212 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52199 + t22379 * t14182 / F::cast_from(24.0_f64) + t26604 * t14193 / F::cast_from(48.0_f64) + t53227 / F::cast_from(384.0_f64) + t55022 - t53234 / F::cast_from(24.0_f64) + t53238 / F::cast_from(192.0_f64);
    t55025
}
