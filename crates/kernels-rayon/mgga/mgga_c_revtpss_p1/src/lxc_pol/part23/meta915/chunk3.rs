//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2952/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2952(t52406: f64, t52407: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64) -> f64 {
    let t78394 = -0.6849333333333333333e-1_f64 * t63338 + 0.2283111111111111111e-1_f64 * t63340 + 0.19025925925925925925e-1_f64 * t63342 + 0.10274e0_f64 * t63361 - 0.6849333333333333333e-1_f64 * t63371 + t52406 - t52407 + 0.17123333333333333333e-1_f64 * t63447 - 0.1522074074074074074e-1_f64 * t63453 + 0.4566222222222222222e-1_f64 * t63459 + 0.11415555555555555555e-1_f64 * t77559 - 0.34246666666666666667e-1_f64 * t77561 + 0.2283111111111111111e0_f64 * t77566 - 0.57077777777777777775e-1_f64 * t77570 - 0.50735802469135802467e-1_f64 * t77575 - 0.22831111111111111111e-1_f64 * t63464 + 0.34246666666666666666e-1_f64 * t77581 - 0.11415555555555555555e-1_f64 * t77586 - 0.41095999999999999999e0_f64 * t77590 + 0.20547999999999999999e0_f64 * t77594;
    t78394
}
