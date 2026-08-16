//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1316/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1316(t24776: f64, t30270: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49393: f64, t49395: f64, t56969: f64, t57027: f64, t57037: f64, t57041: f64) -> f64 {
    let t57513 = -0.34246666666666666665e-1_f64 * t57027 - 0.11415555555555555555e0_f64 * t56969 - 0.3044148148148148148e-1_f64 * t39413 + 0.9132444444444444444e-1_f64 * t39418 + t24776 + 0.4566222222222222222e-1_f64 * t49240 - 0.13698666666666666667e0_f64 * t49242 + 0.22831111111111111111e-1_f64 * t49393 + 0.25367901234567901233e-1_f64 * t49395 + 0.71030123456790123454e-1_f64 * t30270 + 0.2283111111111111111e0_f64 * t57037 - 0.50735802469135802467e-1_f64 * t57041;
    t57513
}
