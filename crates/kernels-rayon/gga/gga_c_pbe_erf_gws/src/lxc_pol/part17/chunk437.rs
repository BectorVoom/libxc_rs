//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 437/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk437(t1651: f64, t592: f64, t587: f64, t1407: f64, t591: f64, t590: f64, t187: f64, t572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1652 = t1651 * t592;
    let t1653 = t587 * t1652;
    let t1654 = 16.0_f64 / 135.0_f64 * t1653;
    let t1655 = t591 * t1407;
    let t1656 = t590 * t1655;
    let t1658 = 4.0_f64 / 45.0_f64 * t587 * t1656;
    let t1660 = 1.0_f64 / t187 / t572;
    (t1652, t1653, t1654, t1655, t1656, t1658, t1660)
}
