//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1254/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1254(t136: f64, t2457: f64, t7769: f64, t93377: f64, t2453: f64, t27212: f64, t25301: f64, t25410: f64, t7774: f64, t93240: f64, t7760: f64, t786: f64, t867: f64) -> (f64, f64, f64, f64, f64) {
    let t99211 = t7769 * t136 * t2457;
    let t99212 = t93377 * t99211;
    let t99257 = t2453 * t27212;
    let t99258 = t99257 * t25301;
    let t99261 = t93240 * t25410 * t7774;
    let t99285 = t786 * t7760 * t867;
    (t99211, t99212, t99258, t99261, t99285)
}
