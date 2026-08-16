//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1255/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1255(t10073: f64, t25403: f64, t27198: f64, t2471: f64, t27202: f64, t15003: f64, t93194: f64, t7759: f64, t822: f64, t2470: f64, t27340: f64, t25387: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99297 = t10073 * t27198 * t25403;
    let t99307 = t27202 * t2471;
    let t99313 = t93194 * t15003;
    let t99334 = t822 * t7759;
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    (t99297, t99307, t99313, t99334, t99365, t99366)
}
