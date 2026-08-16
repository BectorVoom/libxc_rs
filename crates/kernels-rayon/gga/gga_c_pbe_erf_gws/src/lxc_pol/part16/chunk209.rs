//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 209/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk209(t197: f64, t572: f64, t418: f64, t590: f64, t587: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t591 = t197 * t572;
    let t592 = t591 * t418;
    let t593 = t590 * t592;
    let t595 = 4.0_f64 / 45.0_f64 * t587 * t593;
    let t596 = t196 * t196;
    let t597 = 1.0_f64 / t596;
    (t591, t592, t593, t595, t596, t597)
}
