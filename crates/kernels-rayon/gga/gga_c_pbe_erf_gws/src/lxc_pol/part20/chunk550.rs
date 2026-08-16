//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 550/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk550(t75: f64, t959: f64, t472: f64, t414: f64, t960: f64, t409: f64, t1267: f64, t1271: f64, t1394: f64, t1398: f64, t1446: f64, t2510: f64, t2511: f64, t2514: f64, t2516: f64, t2517: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2840 = t959 * t75;
    let t2841 = t2840 * t472;
    let t2842 = 0.58482233974552040708e0_f64 * t2841;
    let t2843 = t414 * t960;
    let t2844 = 4.0_f64 * t2843;
    let t2845 = t409 * t960;
    let t2846 = 4.0_f64 * t2845;
    let t2847 = -t2510 - t1271 - t2511 + t1446 + t2514 + t2516 - t1267 - t1394 - t1398 - t2517 - t2842 - t2844 + t2846;
    (t2840, t2841, t2842, t2843, t2844, t2845, t2846, t2847)
}
