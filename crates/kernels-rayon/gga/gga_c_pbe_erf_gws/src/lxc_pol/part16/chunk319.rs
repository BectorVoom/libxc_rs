//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 319/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk319(t862: f64, t879: f64, t882: f64, t890: f64, t902: f64, t907: f64, t914: f64, t918: f64, t927: f64, t929: f64, t935: f64) -> f64 {
    let t938 = t862 - t879 - t882 - t890 + t902 * t907 / 1536.0_f64 - t914 * t918 / 1536.0_f64 - t927 - t929 * t935 / 768.0_f64;
    t938
}
