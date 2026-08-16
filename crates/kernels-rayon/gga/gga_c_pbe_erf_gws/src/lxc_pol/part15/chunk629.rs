//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 629/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk629(t2928: f64, t3024: f64, t312: f64, t1267: f64, t1271: f64, t1394: f64, t1398: f64, t1446: f64, t2098: f64, t2510: f64, t2511: f64, t2512: f64, t2514: f64, t2516: f64, t2517: f64, t2842: f64, t2844: f64, t2846: f64) -> (f64, f64) {
    let t3025 = t2928 + t3024;
    let t3026 = t3025 * t312;
    let t3027 = -t2510 - t1271 - t2511 + t1446 - t2512 + t2514 + t2516 - t1267 + t2098 - t1394 - t1398 - t2517 - t3026 - t2842 - t2844 + t2846;
    (t3025, t3027)
}
