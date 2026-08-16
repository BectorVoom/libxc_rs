//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1961/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1961(t30681: f64, t72: f64, t1927: f64, t7719: f64, t8143: f64, t2122: f64, t29532: f64, t1923: f64, t2123: f64, t26792: f64, t28154: f64, t29380: f64, t29388: f64, t29412: f64, t29513: f64, t29538: f64, t29544: f64, t29548: f64, t29551: f64, t29554: f64, t29562: f64, t7566: f64, t7702: f64, t7706: f64, t7709: f64, t8144: f64, t8147: f64) -> (f64, f64, f64, f64, f64) {
    let t30682 = t30681 * t72;
    let t30683 = t30682 * t1927;
    let t30686 = t8143 * t7719;
    let t30689 = t2122 * t29532;
    let t30714 = -t29513 * t2123 / 6.0_f64 - t7702 * t8144 / 3.0_f64 - t7702 * t8147 / 3.0_f64 - t1923 * t30683 / 6.0_f64 - t1923 * t30686 / 3.0_f64 - t1923 * t30689 / 6.0_f64 - 5.0_f64 * t26792 * t29562 - 10.0_f64 / 3.0_f64 * t28154 * t29380 + 5.0_f64 / 3.0_f64 * t29388 * t7706 + 2.0_f64 / 3.0_f64 * t29538 * t2123 + 5.0_f64 / 3.0_f64 * t29412 * t7706 + 5.0_f64 / 3.0_f64 * t7566 * t29544 + 5.0_f64 / 6.0_f64 * t7566 * t29548 + t29551 * t2123 / 3.0_f64 + t29554 * t2123 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7709 * t8144 + 2.0_f64 / 3.0_f64 * t7709 * t8147;
    (t30682, t30683, t30686, t30689, t30714)
}
