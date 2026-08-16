//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 766/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk766(t1923: f64, t707: f64, t256: f64, t1914: f64, t1918: f64, t247: f64, t24: f64, t712: f64, t2704: f64, t2718: f64, t248: f64, t1910: f64, t723: f64) -> (f64, f64, f64, f64, f64) {
    let t5416 = t707 * t1923;
    let t5417 = t5416 * t256;
    let t5418 = t1914 * t1918;
    let t5420 = t247 * t1923;
    let t5421 = t24 * t5420;
    let t5423 = 0.18233333333333333333e0_f64 * t712 * t5421;
    let t5426 = 0.10059259259259259259e0_f64 * t2704 - 0.50074074074074074075e0_f64 * t2718;
    let t5427 = t248 * t5426;
    let t5429 = t5427 * t256 / 3.0_f64;
    let t5430 = t1910 * t723;
    (t5417, t5418, t5423, t5429, t5430)
}
