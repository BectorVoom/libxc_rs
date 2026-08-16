//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2127/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2127(t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15125: f64, t15132: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15195: f64, t15200: f64, t15301: f64, t15315: f64, t15322: f64, t15324: f64, t15337: f64) -> f64 {
    let t15339 = 0.264729375e1_f64 * t15108 - 0.157790625e0_f64 * t15111 - 0.3529725e1_f64 * t15114 - 0.17648625e1_f64 * t15116 + 0.6311625e0_f64 * t15119 + 0.31558125e0_f64 * t15121 - 0.11577222222222222222e0_f64 * t15123 - 0.68863333333333333333e0_f64 * t15125 + t15301 - 0.68863333333333333334e0_f64 * t15132 + t15315 - 0.34731666666666666667e-1_f64 * t15178 - 0.46308888888888888889e-1_f64 * t15181 + 0.41678e0_f64 * t15184 + 0.20839e0_f64 * t15187 - 0.22954444444444444444e0_f64 * t15189 + t15322 - 0.516475e0_f64 * t15195 + t15324 - 0.104195e0_f64 * t15200 - 0.13892666666666666667e0_f64 * t11326 + t15337;
    t15339
}
