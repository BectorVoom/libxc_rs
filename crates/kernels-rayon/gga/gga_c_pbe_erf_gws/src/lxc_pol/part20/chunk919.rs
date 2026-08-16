//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 919/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk919(t10247: f64, t10252: f64, t10265: f64, t10270: f64, t10239: f64, t10245: f64, t145: f64, t5726: f64, t5730: f64, t5732: f64, t5733: f64, t8347: f64, t8351: f64, t8371: f64, t8373: f64) -> (f64, f64) {
    let t10272 = t10247 + t10252 + t10265 + t10270;
    let t10275 = -0.31835665774679373271e-1_f64 * t10239 - t8371 - 0.63671331549358746542e-1_f64 * t8373 - 0.31835665774679373271e-1_f64 * t5726 - t5730 - t5732 + 0.3199504064530762818e0_f64 * t5733 + 0.6399008129061525636e0_f64 * t8347 - t8351 - 0.1066501354843587606e0_f64 * t10245 + 0.533250677421793803e-1_f64 * t145 * t10272;
    (t10272, t10275)
}
