//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 721/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk721(t3956: f64, t3980: f64, t1205: f64, t2376: f64, t830: f64, t829: f64) -> (f64, f64, f64, f64) {
    let t4072 = 7.0_f64 / 144.0_f64 * t3956;
    let t4077 = 7.0_f64 / 2304.0_f64 * t3980;
    let t4081 = t2376 * t1205;
    let t4082 = t830 * t4081;
    let t4083 = t829 * t4082;
    (t4072, t4077, t4082, t4083)
}
