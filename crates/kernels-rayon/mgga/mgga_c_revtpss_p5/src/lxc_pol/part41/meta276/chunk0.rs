//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1027/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1027(t10199: f64, t112: f64, t2289: f64, t666: f64, t654: f64, t98: f64, t99: f64, t106: f64, t107: f64, t10: f64, t580: f64, t22: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10201 = 154.0_f64 / 27.0_f64 * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10207 = t654 * t654;
    let t10208 = 1.0_f64 / t10207;
    let t10226 = t99 * t98;
    let t10227 = 1.0_f64 / t10226;
    let t10240 = t107 * t106;
    let t10241 = 1.0_f64 / t10240;
    let t10270 = t10 * t580;
    let t10272 = t576 * t22;
    (t10201, t10202, t10207, t10208, t10227, t10241, t10270, t10272)
}
