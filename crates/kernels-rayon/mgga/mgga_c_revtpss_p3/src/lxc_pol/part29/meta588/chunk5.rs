//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1946/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1946(t28640: f64, t6963: f64, t28141: f64, t7349: f64, t101350: f64, t10309: f64, t25120: f64, t26172: f64, t28147: f64, t33269: f64, t7343: f64, t7709: f64, t7964: f64, t95230: f64, t95241: f64, t95243: f64, t95246: f64, t95248: f64, t95253: f64) -> f64 {
    let t101811 = 32.0_f64 / 9.0_f64 * t6963 * t28640;
    let t101820 = 32.0_f64 / 9.0_f64 * t28141 * t7349;
    let t101824 = -2.0_f64 / 3.0_f64 * t7709 * t26172 - 5.0_f64 / 3.0_f64 * t7343 * t101350 + t101811 - 2.0_f64 / 3.0_f64 * t25120 * t7964 - 8.0_f64 / 9.0_f64 * t95230 - 8.0_f64 / 9.0_f64 * t95241 - 16.0_f64 / 9.0_f64 * t95243 + 176.0_f64 / 27.0_f64 * t95246 + 16.0_f64 / 9.0_f64 * t95248 + t101820 - 40.0_f64 * t10309 * t33269 * t28147 - t95253;
    t101824
}
