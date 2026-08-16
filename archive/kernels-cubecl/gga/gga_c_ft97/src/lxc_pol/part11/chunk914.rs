//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 914/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk914<F: Float>(t37365: F, t37368: F, t37372: F, t37376: F, t37379: F, t37382: F, t37385: F, t37394: F, t37399: F, t37403: F, t37410: F, t37413: F, t37418: F, t37421: F, t37424: F) -> F {
    let t38809 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t37365 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t37368 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t37372 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t37376 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t37379 + F::cast_from(112.0_f64) / F::cast_from(243.0_f64) * t37382 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t37385 - t37394 / F::cast_from(9.0_f64) - F::cast_from(12.0_f64) * t37399 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t37403 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t37410 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t37413 + F::cast_from(2.0_f64) * t37418 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t37421 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t37424;
    t38809
}
