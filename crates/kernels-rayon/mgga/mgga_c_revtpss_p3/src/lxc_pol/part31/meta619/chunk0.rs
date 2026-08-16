//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2067/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2067(t136: f64, t2457: f64, t7769: f64, t93377: f64, t4534: f64, t689: f64, t7014: f64, t27303: f64, t786: f64, t789: f64, t25296: f64, t27216: f64) -> (f64, f64, f64, f64, f64) {
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99216 = 0.10975748638225852664e-1_f64 * t689 * t7014 * t4534;
    let t99219 = 0.19514881078765566038e-1_f64 * t786 * t27303 * t789;
    let t99222 = 0.25702851531048074406e-1_f64 * t27216 * t25296;
    (t99211, t99212, t99216, t99219, t99222)
}
