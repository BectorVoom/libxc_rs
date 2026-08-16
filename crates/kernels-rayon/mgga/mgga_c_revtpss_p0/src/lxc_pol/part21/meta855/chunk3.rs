//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3238/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3238(t1298: f64, t3794: f64, t18134: f64, t5023: f64, t57822: f64, t57825: f64, t57827: f64, t57829: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57842: f64) -> f64 {
    let t60126 = t3794 * t1298;
    let t60130 = 6.0_f64 * t18134 * t5023 * t60126 - t57822 - t57825 + t57827 - t57829 - t57831 - t57833 + t57835 + t57837 - t57840 + t57842;
    t60130
}
