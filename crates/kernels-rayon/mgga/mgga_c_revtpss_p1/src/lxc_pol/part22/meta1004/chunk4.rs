//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3432/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3432(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64) -> f64 {
    let t64458 = 0.1522074074074074074e-1_f64 * t52047 + 0.761037037037037037e-2_f64 * t52049 + 0.12683950617283950617e-1_f64 * t52051 - 0.41096e0_f64 * t63399 - 0.76103703703703703703e-2_f64 * t41330 - 0.50735802469135802469e-2_f64 * t41332 + 0.11415555555555555555e-1_f64 * t63447 - 0.17123333333333333333e-1_f64 * t63451 - 0.50735802469135802467e-2_f64 * t63453 - 0.2283111111111111111e-1_f64 * t63457 + 0.1522074074074074074e-1_f64 * t63459 + 0.68493333333333333332e-1_f64 * t63462 - 0.76103703703703703702e-2_f64 * t63464;
    t64458
}
