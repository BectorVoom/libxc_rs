//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3343/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3343(t41281: f64, t41285: f64, t41287: f64, t41592: f64, t51937: f64, t51942: f64, t63266: f64, t63268: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64) -> f64 {
    let t63295 = -0.21908444444444444444e0_f64 * t51937 + 0.65725333333333333332e0_f64 * t51942 + t41592 + 0.5696775e1_f64 * t63266 - 0.3071625e0_f64 * t63268 + 0.18257037037037037037e0_f64 * t41281 - 0.91285185185185185187e-1_f64 * t41285 - 0.30428395061728395062e-1_f64 * t41287 + 0.11958666666666666667e1_f64 * t63274 - 0.39862222222222222222e0_f64 * t63276 + 0.13287407407407407408e0_f64 * t63278 - 0.39862222222222222222e0_f64 * t63281 - 0.19931111111111111111e0_f64 * t63285 - 0.33218518518518518518e0_f64 * t63290 + 0.11958666666666666667e1_f64 * t63293;
    t63295
}
