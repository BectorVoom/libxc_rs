//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2215/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2215(t25163: f64, t8143: f64, t101226: f64, t2122: f64, t101200: f64, t101230: f64, t25162: f64, t26783: f64, t26786: f64, t26792: f64, t26795: f64, t28119: f64, t28147: f64, t28154: f64, t29380: f64, t7576: f64, t7579: f64, t7709: f64, t92565: f64, t96760: f64, t96765: f64, t96824: f64) -> f64 {
    let t104314 = t8143 * t25163;
    let t104317 = t2122 * t101226;
    let t104330 = 2.0_f64 / 3.0_f64 * t28119 * t7576 + 2.0_f64 / 3.0_f64 * t28119 * t7579 + t7709 * t26783 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7709 * t26786 - 10.0_f64 / 3.0_f64 * t28154 * t96760 - 10.0_f64 / 3.0_f64 * t25162 * t104314 - 10.0_f64 / 3.0_f64 * t25162 * t104317 - 10.0_f64 / 3.0_f64 * t101230 * t26795 - 10.0_f64 * t96824 * t28147 - 10.0_f64 / 3.0_f64 * t92565 * t29380 - 5.0_f64 / 3.0_f64 * t28154 * t96765 - 10.0_f64 * t26792 * t101200;
    t104330
}
