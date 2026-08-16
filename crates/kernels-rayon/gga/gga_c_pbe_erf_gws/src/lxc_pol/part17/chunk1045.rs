//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1045/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1045(t3291: f64, t6416: f64, t2312: f64, t8943: f64, t8948: f64, t8951: f64, t8952: f64, t8954: f64, t8958: f64, t9457: f64, t9460: f64, t9464: f64, t9467: f64, t9470: f64) -> f64 {
    let t9474 = 7.0_f64 / 1152.0_f64 * t6416 * t3291;
    let t9475 = -119.0_f64 / 6912.0_f64 * t9457 - t8943 + t8948 - t2312 * t9460 / 192.0_f64 - t8951 + t8952 - t9464 - t2312 * t9467 / 384.0_f64 - t8954 - t2312 * t9470 / 384.0_f64 - t8958 + t9474;
    t9475
}
