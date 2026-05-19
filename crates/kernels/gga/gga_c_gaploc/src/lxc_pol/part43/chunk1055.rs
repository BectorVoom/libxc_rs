//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1055/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1055<F: Float>(t43444: F, t43447: F, t43448: F, t43449: F, t43450: F, t43455: F, t43465: F, t43468: F, t43471: F, t43477: F, t47193: F, t47196: F, t47199: F, t47203: F, t47206: F, t47212: F, t47215: F, t47222: F, t47227: F, t47230: F) -> F {
    let t51104 = F::cast_from(0.23833659967900284447e0_f64) * t47193 + F::cast_from(0.59584149919750711116e-1_f64) * t47196 - F::cast_from(0.29792074959875355558e-1_f64) * t47199 - F::cast_from(0.79445533226334281487e-1_f64) * t47203 + F::cast_from(0.38342925953920749676e0_f64) * t47206 - F::cast_from(0.76685851907841499352e0_f64) * t47212 - F::cast_from(0.38342925953920749676e0_f64) * t47215 - t43444 - t43447 + t43448 - t43449 + t43450 - t43455 + F::cast_from(0.14300195980740170668e1_f64) * t47222 + F::cast_from(0.14300195980740170668e1_f64) * t47227 + F::cast_from(0.30674340763136599741e2_f64) * t47230 + t43465 + t43468 + t43471 - t43477;
    t51104
}
