//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1623/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1623(t1149: f64, t6474: f64, t12248: f64, t12297: f64, t12397: f64, t16706: f64, t16708: f64, t17010: f64, t17011: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64) -> (f64, f64) {
    let t20580 = t6474 * t1149;
    let t20582 = 0.96491876992155210402e2_f64 * t12248 * t20580;
    let t20597 = -t12397 + 0.76103703703703703703e-2_f64 * t12297 + 0.1522074074074074074e-1_f64 * t16706 + 0.761037037037037037e-2_f64 * t16708 - t17010 - t17011 + 0.3805185185185185185e-2_f64 * t20283 + 0.19025925925925925925e-1_f64 * t20295 - 0.68493333333333333331e-1_f64 * t20300 - 0.2283111111111111111e-1_f64 * t20304 - 0.11415555555555555555e-1_f64 * t20285 + 0.10274e0_f64 * t20308 + 0.68493333333333333332e-1_f64 * t20312 - 0.57077777777777777777e-2_f64 * t20287 - 0.11415555555555555555e-1_f64 * t20315 + 0.34246666666666666666e-1_f64 * t20320 + 0.17123333333333333333e-1_f64 * t20290;
    (t20582, t20597)
}
