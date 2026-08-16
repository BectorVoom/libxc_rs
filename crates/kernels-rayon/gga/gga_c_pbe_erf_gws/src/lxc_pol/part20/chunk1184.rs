//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1184/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1184(t15050: f64, t15057: f64, t15218: f64, t15220: f64, t15222: f64, t15224: f64, t15226: f64, t15228: f64, t15230: f64, t15232: f64, t15234: f64, t15236: f64, t15238: f64, t15241: f64, t15243: f64, t15245: f64) -> f64 {
    let t15247 = t15218 / 768.0_f64 + t15220 / 96.0_f64 - t15222 / 96.0_f64 + t15224 / 768.0_f64 + t15226 / 96.0_f64 - t15228 / 48.0_f64 - t15230 / 768.0_f64 + t15232 / 256.0_f64 - t15234 / 768.0_f64 + t15236 / 24.0_f64 - t15238 / 24.0_f64 - t15241 / 96.0_f64 - t15243 / 768.0_f64 + t15050 - t15057 + t15245 / 96.0_f64;
    t15247
}
