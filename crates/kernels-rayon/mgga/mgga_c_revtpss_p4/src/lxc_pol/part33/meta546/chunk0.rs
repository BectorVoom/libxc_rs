//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1923/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1923(t7575: f64, t7719: f64, t2122: f64, t28089: f64, t28150: f64, t1923: f64, t2123: f64, t25162: f64, t26792: f64, t26795: f64, t28093: f64, t28147: f64, t28154: f64, t29364: f64, t29367: f64, t6954: f64, t6963: f64, t7576: f64, t7579: f64, t7702: f64, t8144: f64, t8147: f64) -> (f64, f64, f64, f64) {
    let t29372 = t7575 * t7719;
    let t29375 = t2122 * t28089;
    let t29380 = t2122 * t28150;
    let t29387 = -t28093 * t2123 / 6.0_f64 - t7702 * t7576 / 6.0_f64 - t7702 * t7579 / 6.0_f64 - t6954 * t8144 / 6.0_f64 - t1923 * t29364 / 6.0_f64 - t1923 * t29367 / 6.0_f64 - t6954 * t8147 / 6.0_f64 - t1923 * t29372 / 6.0_f64 - t1923 * t29375 / 6.0_f64 - 5.0_f64 * t26792 * t28147 - 5.0_f64 / 3.0_f64 * t25162 * t29380 - 5.0_f64 / 3.0_f64 * t28154 * t26795 + t6963 * t8147 / 3.0_f64;
    (t29372, t29375, t29380, t29387)
}
