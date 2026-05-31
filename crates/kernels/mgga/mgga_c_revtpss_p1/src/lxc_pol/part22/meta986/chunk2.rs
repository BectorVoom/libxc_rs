//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3343/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3343<F: Float>(t41281: F, t41285: F, t41287: F, t41592: F, t51937: F, t51942: F, t63266: F, t63268: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F) -> F {
    let t63295 = -F::cast_from(0.21908444444444444444e0_f64) * t51937 + F::cast_from(0.65725333333333333332e0_f64) * t51942 + t41592 + F::cast_from(0.5696775e1_f64) * t63266 - F::cast_from(0.3071625e0_f64) * t63268 + F::cast_from(0.18257037037037037037e0_f64) * t41281 - F::cast_from(0.91285185185185185187e-1_f64) * t41285 - F::cast_from(0.30428395061728395062e-1_f64) * t41287 + F::cast_from(0.11958666666666666667e1_f64) * t63274 - F::cast_from(0.39862222222222222222e0_f64) * t63276 + F::cast_from(0.13287407407407407408e0_f64) * t63278 - F::cast_from(0.39862222222222222222e0_f64) * t63281 - F::cast_from(0.19931111111111111111e0_f64) * t63285 - F::cast_from(0.33218518518518518518e0_f64) * t63290 + F::cast_from(0.11958666666666666667e1_f64) * t63293;
    t63295
}
