//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3430/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3430<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F) -> F {
    let t64430 = -F::cast_from(0.3044148148148148148e-1_f64) * t51973 + F::cast_from(0.35515061728395061727e-1_f64) * t51978 + F::cast_from(0.35515061728395061728e-1_f64) * t41361 + F::cast_from(0.15220740740740740741e-1_f64) * t41363 - F::cast_from(0.761037037037037037e-1_f64) * t63325 + F::cast_from(0.27397333333333333332e0_f64) * t63328 + F::new(0.41096e0) * t63336 - F::cast_from(0.4566222222222222222e-1_f64) * t63338 + F::cast_from(0.1522074074074074074e-1_f64) * t63340 + F::cast_from(0.12683950617283950617e-1_f64) * t63342 - F::cast_from(0.19025925925925925925e-1_f64) * t63346 - F::cast_from(0.50735802469135802467e-1_f64) * t63351 + F::cast_from(0.68493333333333333331e-1_f64) * t63355;
    t64430
}
