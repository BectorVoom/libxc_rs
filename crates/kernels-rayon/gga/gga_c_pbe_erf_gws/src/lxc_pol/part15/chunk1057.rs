//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1057/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1057(t2171: f64, t2345: f64, t9375: f64, t2289: f64, t3283: f64, t2300: f64, t8804: f64, t904: f64, t3242: f64, t6627: f64, t2343: f64, t6592: f64, t6597: f64, t9124: f64, t9129: f64, t9133: f64, t9137: f64, t9138: f64, t9140: f64, t929: f64) -> (f64, f64, f64) {
    let t9588 = t2345 * t9375 * t2171;
    let t9592 = 7.0_f64 / 1152.0_f64 * t2289 * t3283;
    let t9594 = t2300 * t904 * t8804;
    let t9598 = 7.0_f64 / 288.0_f64 * t6627 * t3242;
    let t9599 = -t6592 - t6597 - t9124 + t9129 + t2343 * t9588 / 192.0_f64 + t9133 + t9137 + t9592 + 5.0_f64 / 384.0_f64 * t929 * t9594 + t9138 - t9598 + t9140;
    (t9588, t9594, t9599)
}
