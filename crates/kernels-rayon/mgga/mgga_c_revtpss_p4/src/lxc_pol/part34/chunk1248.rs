//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1248/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1248(t105945: f64, t7063: f64, t7060: f64, t29637: f64, t786: f64, t789: f64, t18797: f64, t25399: f64, t1580: f64, t27194: f64, t689: f64, t29690: f64) -> (f64, f64, f64, f64, f64) {
    let t106387 = t7063 * t105945;
    let t106388 = t106387 * t7060;
    let t106395 = t786 * t29637 * t789;
    let t106407 = t25399 * t18797;
    let t106423 = t689 * t27194 * t1580;
    let t106430 = t29690 * t689;
    (t106388, t106395, t106407, t106423, t106430)
}
