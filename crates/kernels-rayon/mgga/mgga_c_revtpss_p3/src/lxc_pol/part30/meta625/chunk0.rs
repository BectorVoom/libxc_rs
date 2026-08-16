//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2162/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2162(t2470: f64, t27278: f64, t7064: f64, t10073: f64, t25402: f64, t7056: f64, t7759: f64, t136: f64, t2457: f64, t7769: f64, t93377: f64, t4534: f64, t689: f64, t7014: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99201 = t27278 * t2470;
    let t99202 = t7064 * t99201;
    let t99206 = t10073 * t7056 * t25402 * t7759;
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99216 = 0.10975748638225852664e-1_f64 * t689 * t7014 * t4534;
    (t99201, t99202, t99206, t99211, t99212, t99216)
}
