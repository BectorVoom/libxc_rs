//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2177/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2177(t25375: f64, t99161: f64, t1580: f64, t25338: f64, t689: f64, t25365: f64, t27279: f64, t7058: f64, t99201: f64, t99349: f64, t14983: f64, t25399: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99472 = 0.28912093960683998208e-1_f64 * t25375 * t99161;
    let t99475 = 0.10975748638225852664e-1_f64 * t689 * t25338 * t1580;
    let t99480 = 0.25702851531048074406e-1_f64 * t25365 * t27279;
    let t99481 = t7058 * t99201;
    let t99485 = 0.28912093960683998208e-1_f64 * t25375 * t99349;
    let t99487 = 0.19514881078765566038e-1_f64 * t25399 * t14983;
    (t99472, t99475, t99480, t99481, t99485, t99487)
}
