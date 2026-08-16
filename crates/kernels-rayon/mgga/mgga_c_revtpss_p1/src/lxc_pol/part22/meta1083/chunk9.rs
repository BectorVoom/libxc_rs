//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3922/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3922(t1450: f64, t22461: f64, t1353: f64, t21937: f64, t22470: f64, t22475: f64, t3829: f64, t4135: f64, t4139: f64, t47092: f64, t47096: f64, t47098: f64, t49541: f64, t5536: f64, t5541: f64, t74126: f64, t74129: f64, t74131: f64, t74133: f64) -> f64 {
    let t75389 = t22461 * t1450;
    let t75401 = 6.0_f64 * t1353 * t4139 * t75389 + 6.0_f64 * t21937 * t3829 * t5536 + 2.0_f64 * t22475 * t4135 * t5541 + 12.0_f64 * t22470 * t49541 + t47092 - t47096 - t47098 + t74126 - t74129 + t74131 - t74133;
    t75401
}
