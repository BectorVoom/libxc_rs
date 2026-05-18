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
    let t38809 = -F::new(2.0) / F::new(9.0) * t37365 + F::new(8.0) / F::new(9.0) * t37368 + F::new(8.0) / F::new(3.0) * t37372 + F::new(2.0) / F::new(3.0) * t37376 - F::new(16.0) / F::new(27.0) * t37379 + F::new(112.0) / F::new(243.0) * t37382 + F::new(16.0) / F::new(27.0) * t37385 - t37394 / F::new(9.0) - F::new(12.0) * t37399 + F::new(40.0) / F::new(243.0) * t37403 + F::new(40.0) / F::new(27.0) * t37410 + F::new(112.0) / F::new(81.0) * t37413 + F::new(2.0) * t37418 + F::new(16.0) / F::new(9.0) * t37421 + F::new(4.0) / F::new(9.0) * t37424;
    t38809
}
