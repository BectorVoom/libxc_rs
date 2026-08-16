//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 516/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk516(t114: f64, t4287: f64, t655: f64, t2335: f64, t2336: f64, t4261: f64, t4264: f64, t69: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t4288 = t655 * t4287;
    let t4292 = piecewise3(t115, 0.0_f64, t2335 + t2336 / 3.0_f64 + t4261 / 3.0_f64 + t69 * t4264 / 4.0_f64 - t69 * t4288 / 8.0_f64);
    t4292
}
