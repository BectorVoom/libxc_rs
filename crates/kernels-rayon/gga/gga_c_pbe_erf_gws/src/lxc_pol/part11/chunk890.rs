//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 890/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk890(t16740: f64, t197: f64, t155: f64, t1639: f64, t1651: f64, t1802: f64, t5293: f64, t597: f64, t5283: f64, t1642: f64, t212: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17183 = t197 * t16740;
    let t17197 = t155 * t1639;
    let t17252 = t1651 * t1802;
    let t17260 = t5293 * t597;
    let t17268 = t5283 * t597;
    let t17321 = t22 / t212 / t1642;
    (t17183, t17197, t17252, t17260, t17268, t17321)
}
