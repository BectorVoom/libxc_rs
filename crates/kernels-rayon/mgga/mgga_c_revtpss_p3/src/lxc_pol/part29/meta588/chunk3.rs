//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1944/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1944(t114: f64, t101760: f64, t2327: f64, t7968: f64, t26179: f64, t28133: f64, t7706: f64, t95293: f64, t60224: f64, t7342: f64, t13272: f64, t26178: f64, t6960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t101761 = piecewise3(t115, 0.0_f64, t101760);
    let t101767 = t7968 * t2327;
    let t101782 = 80.0_f64 / 9.0_f64 * t26179 * t28133;
    let t101783 = t95293 * t7706;
    let t101785 = t60224 * t7342;
    let t101788 = t13272 * t26178;
    let t101790 = 80.0_f64 / 9.0_f64 * t101788 * t6960;
    (t101761, t101767, t101782, t101783, t101785, t101790)
}
