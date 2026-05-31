//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1326/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1326<F: Float>(t51431: F, t54338: F, t54342: F, t54345: F, t54346: F, t54348: F, t54350: F, t54352: F, t54355: F, t54356: F, t54360: F, t54362: F) -> F {
    let t54364 = -F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t54338 + t54342 / F::cast_from(48.0_f64) - t54345 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t54346 - t54348 / F::cast_from(48.0_f64) - t54350 / F::cast_from(96.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t54352 + t54355 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t54356 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51431 + t54360 / F::cast_from(8.0_f64) + t54362 / F::cast_from(384.0_f64);
    t54364
}
