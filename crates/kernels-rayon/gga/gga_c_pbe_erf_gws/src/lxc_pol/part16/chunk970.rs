//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 970/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk970(t8589: f64, t938: f64, t829: f64, t830: f64, t2373: f64, t3083: f64, t3205: f64, t858: f64, t1118: f64, t810: f64, t353: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8590 = t8589 * t938;
    let t8592 = t829 * t830 * t8590;
    let t8598 = 7.0_f64 / 72.0_f64 * t3083 * t2373;
    let t8599 = t3205 * t858;
    let t8600 = t1118 * t810;
    let t8601 = t353 * t8600;
    let t8602 = t8599 * t8601;
    let t8605 = t858 * t893;
    (t8590, t8592, t8598, t8599, t8602, t8605)
}
