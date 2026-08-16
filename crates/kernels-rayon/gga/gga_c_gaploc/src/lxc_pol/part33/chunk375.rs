//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 375/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk375(t1681: f64, t1700: f64, t1702: f64, t295: f64, t471: f64, t64: f64, t719: f64, t90: f64) -> f64 {
    let t1710 = t1702 * t471 - 4.0_f64 / 3.0_f64 * t719 * t64 + 7.0_f64 / 96.0_f64 * t1681 - 7.0_f64 / 288.0_f64 * t1700 + 4.0_f64 / 3.0_f64 * t295 * t90;
    t1710
}
