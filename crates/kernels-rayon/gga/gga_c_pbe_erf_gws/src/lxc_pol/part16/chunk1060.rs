//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1060/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1060(t2272: f64, t3252: f64, t2253: f64, t2312: f64, t6275: f64, t6628: f64, t6637: f64, t6656: f64, t9142: f64, t9143: f64, t9145: f64, t9174: f64, t9601: f64, t9604: f64, t9609: f64, t9612: f64, t9616: f64) -> (f64, f64) {
    let t9619 = t3252 * t2272;
    let t9623 = -t9601 - t9142 - t9143 - t9145 - 7.0_f64 / 288.0_f64 * t6628 - t2253 * t9604 / 768.0_f64 + t6275 * t9609 / 96.0_f64 + t6637 * t9612 / 768.0_f64 - t6637 * t9616 / 384.0_f64 + t9174 - t2312 * t9619 / 384.0_f64 - 35.0_f64 / 1152.0_f64 * t6656;
    (t9619, t9623)
}
