//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2887/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2887(t52035: f64, t52037: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64) -> f64 {
    let t52406 = 0.4566222222222222222e-1_f64 * t52035;
    let t52407 = 0.1522074074074074074e-1_f64 * t52037;
    let t52426 = t52406 - t52407 - 0.68493333333333333331e-1_f64 * t52039 - 0.34246666666666666665e-1_f64 * t52041 - 0.6849333333333333333e-1_f64 * t52045 + 0.2283111111111111111e-1_f64 * t52047 + 0.11415555555555555555e-1_f64 * t52049 + 0.19025925925925925925e-1_f64 * t52051 - 0.34246666666666666665e-1_f64 * t52054 - 0.34246666666666666665e-1_f64 * t52057 - 0.57077777777777777775e-1_f64 * t52060 - 0.30822e0_f64 * t52063 - 0.34246666666666666665e-1_f64 * t41365 + 0.11415555555555555555e-1_f64 * t41367 + 0.34246666666666666665e-1_f64 * t41308 - 0.2283111111111111111e-1_f64 * t41330 - 0.1522074074074074074e-1_f64 * t41332 + 0.57077777777777777777e-2_f64 * t41334 + 0.63419753086419753085e-2_f64 * t41336 - 0.30822e0_f64 * t52112;
    t52426
}
