//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1976/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1976(t13272: f64, t7565: f64, t38: f64, t8142: f64, t2247: f64, t2123: f64, t26749: f64, t26755: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28133: f64, t28141: f64, t6960: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64, t7706: f64, t7709: f64, t8144: f64) -> (f64, f64, f64, f64) {
    let t29388 = t13272 * t7565;
    let t29411 = t38 * t8142;
    let t29412 = t2247 * t29411;
    let t29419 = 5.0_f64 / 6.0_f64 * t29388 * t6960 + t28141 * t2123 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t26749 * t7706 + 5.0_f64 / 6.0_f64 * t26755 * t7706 + 5.0_f64 / 6.0_f64 * t7566 * t28105 + 5.0_f64 / 6.0_f64 * t7566 * t28109 + t28112 * t2123 / 3.0_f64 + t28116 * t2123 / 3.0_f64 + t28119 * t2123 / 3.0_f64 + t7709 * t7576 / 3.0_f64 + t7709 * t7579 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t29412 * t6960 + t6963 * t8144 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t7566 * t28133;
    (t29388, t29411, t29412, t29419)
}
