//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2076/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2076(t7058: f64, t99201: f64, t25375: f64, t99349: f64, t14983: f64, t25399: f64, t7064: f64, t99321: f64, t25411: f64, t99389: f64, t2435: f64, t7774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99481 = t7058 * t99201;
    let t99485 = 0.28912093960683998208e-1_f64 * t25375 * t99349;
    let t99487 = 0.19514881078765566038e-1_f64 * t25399 * t14983;
    let t99491 = 0.25702851531048074406e-1_f64 * t7064 * t99321;
    let t99493 = 0.25702851531048074406e-1_f64 * t25411 * t99389;
    let t99495 = t7774 * t2435;
    (t99481, t99485, t99487, t99491, t99493, t99495)
}
