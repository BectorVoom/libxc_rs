//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1296/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1296(t116: f64, t26799: f64, t2327: f64, t7583: f64, t10368: f64, t55: f64, t10326: f64, t10356: f64, t11231: f64, t1923: f64, t1927: f64, t2122: f64, t2123: f64, t25117: f64, t25150: f64, t26776: f64, t26782: f64, t26783: f64, t26786: f64, t26789: f64, t6954: f64, t6977: f64, t72: f64, t7571: f64, t7576: f64, t7579: f64, t92612: f64, t92628: f64, t92632: f64) -> (f64, f64, f64) {
    let t96706 = t26799 * t116;
    let t96709 = t7583 * t2327;
    let t96733 = t55 * t10368;
    let t96748 = -t1923 * t2122 * t92628 / 6.0_f64 + t25117 * t7576 + t25117 * t7579 - t92632 * t2123 / 6.0_f64 - t25150 * t7576 / 2.0_f64 - t25150 * t7579 / 2.0_f64 - t6954 * t26783 / 2.0_f64 - t6954 * t26786 - t6954 * t26789 / 2.0_f64 - t1923 * (5.0_f64 / 108.0_f64 * t96733 * t10356 + 5.0_f64 / 6.0_f64 * t26776 * t11231 - 5.0_f64 / 6.0_f64 * t7571 * t10326 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t26782 * t6977 / 2.0_f64;
    (t96706, t96709, t96748)
}
