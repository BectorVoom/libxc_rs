//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1180/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1180(t138: f64, t9675: f64, t9674: f64, t4075: f64, t556: f64, t786: f64, t1444: f64, t2434: f64, t123: f64, t3915: f64, t1359: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9691 = 0.17073386770573548589e-1_f64 * t9292 * t1359;
    (t9676, t9677, t9680, t9685, t9686, t9687, t9691)
}
