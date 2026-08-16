//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1090/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1090(t12166: f64, t12171: f64, t12182: f64, t12187: f64, t12191: f64, t12195: f64, t335: f64, t6731: f64, t6793: f64, t844: f64, t8602: f64, t8629: f64, t8690: f64, t8700: f64, t8716: f64, t8793: f64, t9249: f64, t9253: f64, t9272: f64, t9275: f64, t9289: f64, t9290: f64) -> f64 {
    let t12197 = t335 * t12166 / 96.0_f64 - t844 * t12171 / 48.0_f64 + t8629 * t8690 / 48.0_f64 + t8793 * t8602 / 8.0_f64 + t8793 * t8716 / 24.0_f64 + t6793 * t12182 / 24.0_f64 + t8629 * t8700 / 24.0_f64 - t9249 + t9253 - t6731 - 7.0_f64 / 48.0_f64 * t12187 - t9272 + 35.0_f64 / 216.0_f64 * t9275 + t9289 - t844 * t12191 / 48.0_f64 - 35.0_f64 / 108.0_f64 * t9290 - 7.0_f64 / 144.0_f64 * t12195;
    t12197
}
