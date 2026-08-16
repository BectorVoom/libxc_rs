//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 965/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk965(t4077: f64, t676: f64, t123: f64, t9680: f64, t1444: f64, t2434: f64, t3915: f64, t1359: f64, t9292: f64, t1363: f64, t9288: f64, t1362: f64) -> (f64, f64, f64, f64) {
    let t9681 = t676 * t4077;
    let t9682 = t123 * t9681;
    let t9683 = t9680 * t9682;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9691 = 0.17073386770573548589e-1_f64 * t9292 * t1359;
    let t9692 = t1363 * t9288;
    let t9694 = 0.30356481678079769392e-1_f64 * t1362 * t9692;
    (t9683, t9687, t9691, t9694)
}
