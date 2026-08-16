//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 920/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk920(t39296: f64, t39319: f64, t42913: f64, t45316: f64, t45318: f64, t45323: f64, t45325: f64, t45327: f64, t45329: f64, t45331: f64, t45333: f64, t45337: f64, t45339: f64, t45341: f64, t45345: f64, t45349: f64, t45355: f64) -> f64 {
    let t45357 = 0.31923449919973379548e-4_f64 * t45316 + t39296 - 0.19863479950205658386e-4_f64 * t45318 - t42913 - 0.15961724959986689774e-4_f64 * t45323 + 0.25538759935978703639e-4_f64 * t45325 + 0.85129199786595678796e-5_f64 * t45327 - 0.85129199786595678796e-5_f64 * t45329 - 0.59590439850616975155e-4_f64 * t45331 + 0.59590439850616975155e-4_f64 * t45333 - 0.12769379967989351819e-4_f64 * t45337 + 0.12769379967989351819e-4_f64 * t45339 + t39319 - 0.19863479950205658386e-4_f64 * t45341 - 0.85129199786595678796e-5_f64 * t45345 - 0.42564599893297839398e-5_f64 * t45349 + 0.42564599893297839398e-5_f64 * t45355;
    t45357
}
