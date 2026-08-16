//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2886/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2886(t51973: f64, t41361: f64, t41363: f64, t41369: f64, t41908: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64, t52028: f64, t52031: f64, t52033: f64) -> f64 {
    let t52397 = 0.2283111111111111111e-1_f64 * t51973;
    let t52405 = 0.41096e0_f64 * t51849 - 0.11415555555555555555e-1_f64 * t51853 - 0.50735802469135802467e-1_f64 * t51858 + 0.10274e0_f64 * t51863 + 0.10274e0_f64 * t51867 + 0.34246666666666666666e-1_f64 * t51871 - 0.41095999999999999999e0_f64 * t51875 + t41908 + 0.20547999999999999999e0_f64 * t51961 - 0.57077777777777777775e-1_f64 * t51965 + 0.17123333333333333333e-1_f64 * t51967 - 0.17123333333333333333e-1_f64 * t51971 - t52397 + 0.17757530864197530864e-1_f64 * t51978 + 0.53272592592592592591e-1_f64 * t41361 + 0.4566222222222222222e-1_f64 * t41363 - 0.2283111111111111111e-1_f64 * t41369 + 0.20547999999999999999e0_f64 * t52028 + 0.2283111111111111111e0_f64 * t52031 + 0.10274e0_f64 * t52033;
    t52405
}
