//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 920/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk920<F: Float>(t39296: F, t39319: F, t42913: F, t45316: F, t45318: F, t45323: F, t45325: F, t45327: F, t45329: F, t45331: F, t45333: F, t45337: F, t45339: F, t45341: F, t45345: F, t45349: F, t45355: F) -> F {
    let t45357 = F::cast_from(0.31923449919973379548e-4_f64) * t45316 + t39296 - F::cast_from(0.19863479950205658386e-4_f64) * t45318 - t42913 - F::cast_from(0.15961724959986689774e-4_f64) * t45323 + F::cast_from(0.25538759935978703639e-4_f64) * t45325 + F::cast_from(0.85129199786595678796e-5_f64) * t45327 - F::cast_from(0.85129199786595678796e-5_f64) * t45329 - F::cast_from(0.59590439850616975155e-4_f64) * t45331 + F::cast_from(0.59590439850616975155e-4_f64) * t45333 - F::cast_from(0.12769379967989351819e-4_f64) * t45337 + F::cast_from(0.12769379967989351819e-4_f64) * t45339 + t39319 - F::cast_from(0.19863479950205658386e-4_f64) * t45341 - F::cast_from(0.85129199786595678796e-5_f64) * t45345 - F::cast_from(0.42564599893297839398e-5_f64) * t45349 + F::cast_from(0.42564599893297839398e-5_f64) * t45355;
    t45357
}
