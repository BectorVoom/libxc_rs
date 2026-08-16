//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1338/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1338(t16170: f64, t2853: f64, t1042: f64, t15100: f64, t15103: f64, t15377: f64, t15379: f64, t15382: f64, t15385: f64, t15388: f64, t15392: f64, t15395: f64, t15519: f64, t15522: f64, t15524: f64, t15528: f64, t15530: f64, t15536: f64, t15540: f64, t15545: f64) -> (f64, f64) {
    let t16171 = t16170 * t2853;
    let t16172 = t1042 * t16171;
    let t16179 = t15519 + t15522 + t15100 - t15103 - t15524 - t15528 + t15530 - t15536 + t15540 - t15545 - t15377 + t15379 - t15382 - t15385 - t15388 + t15392 + t15395;
    (t16172, t16179)
}
