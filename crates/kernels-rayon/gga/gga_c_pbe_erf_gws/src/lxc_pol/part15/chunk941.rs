//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 941/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk941(t1503: f64, t522: f64, t524: f64, t2913: f64, t5651: f64, t100: f64, t1504: f64, t2921: f64, t475: f64, t1076: f64, t39: f64, t2848: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8331 = t1503 * t522 * t524;
    let t8332 = t5651 * t2913;
    let t8335 = t1504 * t100;
    let t8341 = t475 * t2921;
    let t8347 = t39 * t1076;
    let t8351 = 0.2133002709687175212e0_f64 * t532 * t2848;
    (t8331, t8332, t8335, t8341, t8347, t8351)
}
