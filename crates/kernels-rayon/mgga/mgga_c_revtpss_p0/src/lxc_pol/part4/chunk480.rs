//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 480/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk480(t265: f64, t393: f64, t1076: f64, t1647: f64, t1652: f64, t1680: f64, t1696: f64, t342: f64, t386: f64, t995: f64, t1102: f64, t1587: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t198: f64, t336: f64) -> (f64, f64) {
    let t394 = t265 < t393;
    let t1699 = 0.65854491829355115987e0_f64 * t1647 * t386 - 0.65854491829355115987e0_f64 * t995 * t1652 + 0.65854491829355115987e0_f64 * t342 * t1680 - 0.65854491829355115987e0_f64 * t1076 * t1696;
    let t1704 = piecewise3(t394, t1102 * t1699 * t198 * t336 - t1598 + t1612 + t1638 + t1640 - t1644, t1587);
    (t1699, t1704)
}
