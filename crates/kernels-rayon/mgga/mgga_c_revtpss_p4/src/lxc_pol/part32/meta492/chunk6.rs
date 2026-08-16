//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1758/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1758(t26179: f64, t7706: f64, t7349: f64, t7709: f64, t13272: f64, t7342: f64, t2048: f64, t26180: f64, t26185: f64, t26187: f64, t28105: f64, t28109: f64, t28112: f64, t28116: f64, t28119: f64, t28141: f64, t6960: f64, t7343: f64, t7352: f64) -> (f64, f64, f64, f64) {
    let t28598 = t26179 * t7706;
    let t28600 = t7709 * t7349;
    let t28602 = t13272 * t7342;
    let t28621 = 40.0_f64 / 9.0_f64 * t26180 + 16.0_f64 / 9.0_f64 * t26185 + 40.0_f64 / 9.0_f64 * t28598 + 16.0_f64 / 9.0_f64 * t28600 - 5.0_f64 / 3.0_f64 * t28602 * t6960 - 2.0_f64 / 3.0_f64 * t28141 * t2048 - 5.0_f64 / 3.0_f64 * t26187 * t7706 - 5.0_f64 / 3.0_f64 * t7343 * t28105 - 5.0_f64 / 3.0_f64 * t7343 * t28109 - 2.0_f64 / 3.0_f64 * t28112 * t2048 - 2.0_f64 / 3.0_f64 * t28116 * t2048 - 2.0_f64 / 3.0_f64 * t28119 * t2048 - 2.0_f64 / 3.0_f64 * t7709 * t7352;
    (t28598, t28600, t28602, t28621)
}
