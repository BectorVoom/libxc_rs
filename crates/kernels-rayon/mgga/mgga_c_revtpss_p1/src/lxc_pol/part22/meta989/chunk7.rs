//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3367/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367(t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64) -> f64 {
    let t63440 = -4.0_f64 / 9.0_f64 * t63359 + 4.0_f64 / 3.0_f64 * t63361 + 4.0_f64 / 3.0_f64 * t63366 - 2.0_f64 * t63369 - 8.0_f64 / 9.0_f64 * t63371 - 2.0_f64 * t63374 + 4.0_f64 / 3.0_f64 * t52033 + 32.0_f64 / 27.0_f64 * t52035 - 32.0_f64 / 81.0_f64 * t52037 - 8.0_f64 / 9.0_f64 * t52039 - 4.0_f64 / 9.0_f64 * t52041 - 8.0_f64 / 9.0_f64 * t52045;
    t63440
}
