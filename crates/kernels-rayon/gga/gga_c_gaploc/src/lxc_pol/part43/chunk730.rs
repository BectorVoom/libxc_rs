//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 730/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk730(t13015: f64, t13018: f64, t13026: f64, t13028: f64, t13036: f64, t13040: f64, t13044: f64, t13047: f64, t13050: f64, t13849: f64, t13852: f64, t13855: f64) -> f64 {
    let t14498 = -t13015 - t13018 + t13026 + t13028 + t13036 - t13040 + t13044 - t13047 + 0.38342925953920749676e0_f64 * t13849 - 0.38342925953920749676e0_f64 * t13852 + t13855 - t13050;
    t14498
}
