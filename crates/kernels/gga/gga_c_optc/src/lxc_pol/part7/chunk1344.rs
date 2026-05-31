//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1344/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1344<F: Float>(t26252: F, t26258: F, t26278: F, t26280: F, t26284: F, t26289: F, t26293: F, t26296: F, t26300: F, t26304: F, t26306: F, t26261: F) -> (F, F) {
    let t26833 = F::cast_from(0.26382716049382716049e-1_f64) * t26252 + F::cast_from(0.23744444444444444444e0_f64) * t26258 - F::cast_from(0.11872222222222222222e0_f64) * t26278 + F::cast_from(0.14246666666666666667e0_f64) * t26280 - F::cast_from(0.42739999999999999999e0_f64) * t26284 + F::cast_from(0.42739999999999999999e0_f64) * t26289 - F::cast_from(0.35616666666666666666e-1_f64) * t26293 + F::cast_from(0.4274e0_f64) * t26296 - F::cast_from(0.6411e0_f64) * t26300 + F::cast_from(0.10685e0_f64) * t26304 - F::cast_from(0.14246666666666666667e0_f64) * t26306;
    let t26836 = F::cast_from(0.18467901234567901234e0_f64) * t26261;
    (t26833, t26836)
}
