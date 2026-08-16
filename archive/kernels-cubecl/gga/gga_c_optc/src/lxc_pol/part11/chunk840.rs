//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 840/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk840<F: Float>(t16287: F, t197: F, t10048: F, t13536: F, t13538: F, t13543: F, t193: F, t4752: F, t6653: F, t750: F, t201: F, t5: F) -> (F, F, F) {
    let t16288 = t197 * t16287;
    let t16292 = t6653 - F::cast_from(2200.0_f64) / F::cast_from(27.0_f64) * t10048 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t13536 + F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t13543 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t193 * t13538 * t4752 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t193 * t750 * t16288;
    let t16294 = t5 * t16292 * t201;
    (t16288, t16292, t16294)
}
