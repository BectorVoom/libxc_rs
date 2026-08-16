//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1360/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1360(t15482: f64, t2376: f64, t829: f64, t830: f64, t11401: f64, t14185: f64, t14894: f64, t14918: f64, t14958: f64, t2408: f64, t2498: f64, t3207: f64, t35566: f64, t55077: f64, t55087: f64, t56323: f64, t56333: f64, t56337: f64, t56341: f64, t56343: f64, t56349: f64, t56351: f64, t56357: f64, t827: f64, t9283: f64) -> f64 {
    let t58103 = t2376 * t15482;
    let t58105 = t829 * t830 * t58103;
    let t58110 = -7.0_f64 / 1152.0_f64 * t56323 + t56333 / 384.0_f64 - t55077 + t56337 / 192.0_f64 + t56341 / 192.0_f64 + 7.0_f64 / 2304.0_f64 * t56343 + 7.0_f64 / 2304.0_f64 * t56349 + t56351 / 48.0_f64 - t3207 * t35566 * t14894 / 8.0_f64 + t55087 - t2408 * t9283 * t14185 * t11401 / 12.0_f64 - t2408 * t35566 * t14958 / 12.0_f64 - 7.0_f64 / 288.0_f64 * t56357 - t827 * t58105 / 96.0_f64 - t2498 * t14918 / 48.0_f64;
    t58110
}
