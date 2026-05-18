//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1287/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1287<F: Float>(t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F, t522: F, t8656: F) -> (F, F) {
    let t26308 = F::new(40.0) / F::new(81.0) * t26252 + F::new(40.0) / F::new(9.0) * t26258 - F::new(20.0) / F::new(9.0) * t26278 + F::new(8.0) / F::new(3.0) * t26280 - F::new(8.0) * t26284 + F::new(8.0) * t26289 - F::new(2.0) / F::new(3.0) * t26293 + F::new(8.0) * t26296 - F::new(12.0) * t26300 + F::new(2.0) * t26304 - F::new(8.0) / F::new(3.0) * t26306;
    let t26309 = t522 * t8656;
    (t26308, t26309)
}
