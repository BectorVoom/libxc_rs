//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 502/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk502(t2376: f64, t938: f64, t830: f64, t829: f64, t2306: f64, t825: f64, t2271: f64, t376: f64, t891: f64) -> (f64, f64, f64, f64, f64) {
    let t2377 = t2376 * t938;
    let t2378 = t830 * t2377;
    let t2379 = t829 * t2378;
    let t2383 = t2306 * t825;
    let t2391 = t2271 * t825;
    let t2395 = t891 * t376;
    (t2377, t2379, t2383, t2391, t2395)
}
