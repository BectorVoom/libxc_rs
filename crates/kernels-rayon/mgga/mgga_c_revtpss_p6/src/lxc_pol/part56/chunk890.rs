//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 890/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk890(t1479: f64, t60: f64, t25137: f64, t26776: f64, t4181: f64, t4186: f64, t606: f64, t7571: f64, t72: f64, t1927: f64, t6977: f64, t8143: f64) -> (f64, f64, f64) {
    let t29355 = t1479 * t60;
    let t29362 = 20.0_f64 / 9.0_f64 * t29355 * t606 + 5.0_f64 / 18.0_f64 * t26776 * t4181 - 5.0_f64 / 6.0_f64 * t7571 * t4186 - t25137;
    let t29363 = t29362 * t72;
    let t29364 = t29363 * t1927;
    let t29367 = t8143 * t6977;
    (t29362, t29364, t29367)
}
