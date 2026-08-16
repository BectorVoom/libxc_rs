//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3430/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64) -> f64 {
    let t64430 = -0.3044148148148148148e-1_f64 * t51973 + 0.35515061728395061727e-1_f64 * t51978 + 0.35515061728395061728e-1_f64 * t41361 + 0.15220740740740740741e-1_f64 * t41363 - 0.761037037037037037e-1_f64 * t63325 + 0.27397333333333333332e0_f64 * t63328 + 0.41096e0_f64 * t63336 - 0.4566222222222222222e-1_f64 * t63338 + 0.1522074074074074074e-1_f64 * t63340 + 0.12683950617283950617e-1_f64 * t63342 - 0.19025925925925925925e-1_f64 * t63346 - 0.50735802469135802467e-1_f64 * t63351 + 0.68493333333333333331e-1_f64 * t63355;
    t64430
}
