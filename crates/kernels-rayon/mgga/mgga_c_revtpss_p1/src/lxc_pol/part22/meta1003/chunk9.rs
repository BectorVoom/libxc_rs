//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3427/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3427(t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64) -> f64 {
    let t64386 = -0.23744444444444444444e-1_f64 * t63359 + 0.71233333333333333333e-1_f64 * t63361 + 0.71233333333333333332e-1_f64 * t63366 - 0.10685e0_f64 * t63369 - 0.47488888888888888888e-1_f64 * t63371 - 0.10685e0_f64 * t63374 + 0.71233333333333333332e-1_f64 * t52033 + 0.63318518518518518517e-1_f64 * t52035 - 0.21106172839506172839e-1_f64 * t52037 - 0.47488888888888888888e-1_f64 * t52039 - 0.23744444444444444444e-1_f64 * t52041 - 0.47488888888888888888e-1_f64 * t52045;
    t64386
}
