//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1363/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1363(t15513: f64, t840: f64, t1115: f64, t27047: f64, t3067: f64, t54918: f64, t55110: f64, t55192: f64, t55195: f64, t55198: f64, t55204: f64, t55734: f64, t56505: f64, t56511: f64, t56514: f64, t56520: f64, t56525: f64, t56534: f64, t56545: f64, t58050: f64, t8629: f64, t8793: f64, t938: f64) -> f64 {
    let t58176 = t840 * t15513;
    let t58196 = t56505 / 96.0_f64 - t1115 * t54918 / 48.0_f64 + 7.0_f64 / 288.0_f64 * t58176 - 35.0_f64 / 216.0_f64 * t55192 + t55195 - t56511 / 384.0_f64 + t55198 - t56514 / 12.0_f64 - t8629 * t27047 * t3067 * t58050 * t938 / 48.0_f64 - t8629 * t55110 / 24.0_f64 + t56520 / 768.0_f64 - t8793 * t55204 / 8.0_f64 - t8793 * t55734 / 12.0_f64 - t56525 / 768.0_f64 - t56534 / 384.0_f64 - t56545 / 192.0_f64;
    t58196
}
