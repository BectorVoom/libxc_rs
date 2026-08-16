//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1278/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1278(t4131: f64, t676: f64, t123: f64, t3915: f64, t2453: f64, t3914: f64, t1444: f64, t2438: f64, t138: f64, t4075: f64, t556: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9670 = t676 * t4131;
    let t9671 = t123 * t9670;
    let t9672 = t3915 * t9671;
    let t9674 = t2453 * t3914;
    let t9675 = t2438 * t1444;
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    (t9671, t9672, t9674, t9675, t9676, t9677, t9680)
}
