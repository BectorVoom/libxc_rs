//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2196/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2196(t1493: f64, t2248: f64, t77: f64, t2315: f64, t2259: f64, t4173: f64, t38: f64, t60248: f64, t2251: f64, t28104: f64, t644: f64, t2014: f64, t25177: f64, t7934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101337 = t77 * t1493 * t2248;
    let t101350 = t77 * t1493 * t2315;
    let t101357 = t4173 * t2259;
    let t101360 = t60248 * t38;
    let t101376 = t4173 * t2251;
    let t101399 = t77 * t28104 * t644;
    let t101416 = 2.0_f64 * t2014 * t7934 * t25177;
    (t101337, t101350, t101357, t101360, t101376, t101399, t101416)
}
