//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1043/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1043(t2300: f64, t8759: f64, t904: f64, t2253: f64, t2277: f64, t8925: f64, t8927: f64, t8930: f64, t8932: f64, t8936: f64, t8938: f64, t914: f64, t929: f64, t9434: f64, t9438: f64, t9443: f64, t9447: f64, t9449: f64) -> (f64, f64) {
    let t9453 = t2300 * t904 * t8759;
    let t9456 = -t8925 - t2253 * t9434 / 384.0_f64 - t8927 - t8930 + t8932 - t2277 * t9438 / 768.0_f64 + t2277 * t9443 / 768.0_f64 + t8936 - t8938 + t9447 - t914 * t9449 / 1536.0_f64 + 5.0_f64 / 768.0_f64 * t929 * t9453;
    (t9453, t9456)
}
