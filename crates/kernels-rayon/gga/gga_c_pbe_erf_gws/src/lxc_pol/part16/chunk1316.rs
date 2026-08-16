//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1316/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1316(t1161: f64, t1206: f64, t14240: f64, t14250: f64, t14881: f64, t2409: f64, t2417: f64, t3066: f64, t3067: f64, t3207: f64, t4227: f64, t53323: f64, t53327: f64, t53338: f64, t55059: f64, t55062: f64, t55065: f64, t55074: f64, t55077: f64, t55087: f64, t55090: f64, t6793: f64, t8589: f64, t8647: f64, t8759: f64, t9283: f64, t9296: f64) -> f64 {
    let t55093 = -t3207 * t2409 * t8589 * t14250 / 16.0_f64 - t3066 * t2409 * t9296 * t4227 * t2417 / 16.0_f64 + 35.0_f64 / 216.0_f64 * t55059 - t55062 - t53323 / 384.0_f64 + t6793 * t55065 / 24.0_f64 - t53327 / 192.0_f64 + t3066 * t2409 * t3067 * t14240 * t1161 / 48.0_f64 - t55074 + t53338 / 768.0_f64 - t55077 - t3207 * t9283 * t1206 * t8759 / 16.0_f64 - t3066 * t9283 * t14881 * t8647 / 8.0_f64 + t55087 - t6793 * t55090 / 12.0_f64;
    t55093
}
