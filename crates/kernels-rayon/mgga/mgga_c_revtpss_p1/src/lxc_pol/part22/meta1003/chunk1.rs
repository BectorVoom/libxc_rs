//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3419/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3419(t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64, t63469: f64, t63471: f64) -> f64 {
    let t64261 = 0.18363555555555555555e1_f64 * t52035 - 0.6121185185185185185e0_f64 * t52037 - 0.13772666666666666666e1_f64 * t52039 - 0.68863333333333333332e0_f64 * t52041 - 0.13772666666666666666e1_f64 * t52045 + 0.45908888888888888888e0_f64 * t52047 + 0.22954444444444444444e0_f64 * t52049 + 0.38257407407407407407e0_f64 * t52051 + 0.13892666666666666667e0_f64 * t52065 - 0.18523555555555555556e0_f64 * t63393 + 0.6311625e0_f64 * t63396 - 0.123954e2_f64 * t63399 + 0.6311625e0_f64 * t63469 + 0.264729375e1_f64 * t63471;
    t64261
}
