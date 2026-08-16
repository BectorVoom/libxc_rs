//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 941/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk941(t159: f64, t3096: f64, t9213: f64, t395: f64, t402: f64, t392: f64, t2909: f64, t404: f64, t394: f64, t9181: f64, t2997: f64, t430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9230 = t159 * t3096;
    let t9243 = 28.0_f64 / 27.0_f64 * t9213;
    let t9267 = 1.0_f64 / t395 / t402 / 4.0_f64;
    let t9271 = 1.0_f64/pow_3_2(t392);
    let t9291 = 1.0_f64 / t2909 / t404;
    let t9292 = t394 * t9291;
    let t9297 = 0.36514074074074074075e0_f64 * t9181;
    let t9306 = 0.93011851851851851854e0_f64 * t9213;
    let t9331 = 0.28842592592592592592e-1_f64 * t9213;
    let t9347 = 1.0_f64 / t2997 / t430;
    (t9230, t9243, t9267, t9271, t9292, t9297, t9306, t9331, t9347)
}
