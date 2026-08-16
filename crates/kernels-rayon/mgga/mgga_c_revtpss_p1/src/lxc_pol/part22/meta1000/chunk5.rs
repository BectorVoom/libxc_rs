//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3403/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3403(t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64) -> f64 {
    let t63875 = -0.12361111111111111111e-1_f64 * t63359 + 0.37083333333333333334e-1_f64 * t63361 + 0.37083333333333333333e-1_f64 * t63366 - 0.55625000000000000001e-1_f64 * t63369 - 0.24722222222222222222e-1_f64 * t63371 - 0.55625000000000000001e-1_f64 * t63374 + 0.37083333333333333333e-1_f64 * t52033 + 0.32962962962962962963e-1_f64 * t52035 - 0.10987654320987654321e-1_f64 * t52037 - 0.24722222222222222222e-1_f64 * t52039 - 0.12361111111111111111e-1_f64 * t52041 - 0.24722222222222222223e-1_f64 * t52045;
    t63875
}
