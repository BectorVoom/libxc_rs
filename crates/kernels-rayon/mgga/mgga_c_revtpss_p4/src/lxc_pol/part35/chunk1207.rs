//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1207/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1207(t101783: f64, t101793: f64, t108966: f64, t108990: f64, t110039: f64, t110044: f64, t114246: f64, t114264: f64, t114322: f64, t114349: f64, t2048: f64, t26175: f64, t28154: f64, t28628: f64, t29551: f64, t7964: f64, t95316: f64) -> f64 {
    let t115291 = 20.0_f64 * t108966 * t28628 + 20.0_f64 * t28154 * t110039 + 30.0_f64 * t26175 * t114246 + 10.0_f64 * t28154 * t110044 + 10.0_f64 * t108990 * t28628 - 2.0_f64 * t29551 * t7964 - 2.0_f64 * t114322 * t2048 - 440.0_f64 / 9.0_f64 * t101783 - 176.0_f64 / 9.0_f64 * t101793 - 70.0_f64 * t95316 * t114264 + t114349 * t2048 / 3.0_f64;
    t115291
}
