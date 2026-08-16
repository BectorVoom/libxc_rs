//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 460/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk460(t339: f64, t816: f64, t19: f64, t793: f64, t796: f64, t801: f64, t116: f64, t299: f64, t799: f64, t798: f64, t814: f64, t817: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2085 = t816 * t339;
    let t2092 = t793 * t796 * t19;
    let t2093 = t2092 * t801;
    let t2096 = t799 * t299 * t116;
    let t2097 = t798 * t2096;
    let t2098 = 0.6846054806677777778e0_f64 * t2097;
    let t2102 = t814 * t817;
    (t2085, t2092, t2093, t2096, t2098, t2102)
}
