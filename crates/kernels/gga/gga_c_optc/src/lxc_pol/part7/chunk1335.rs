//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1335/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1335<F: Float>(t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26324: F, t26394: F, t26396: F, t26398: F, t26406: F, t26409: F, t26412: F, t26415: F) -> F {
    let t26642 = -F::cast_from(0.13772666666666666667e1_f64) * t26324 - F::cast_from(0.166712e1_f64) * t26394 + F::cast_from(0.27785333333333333333e0_f64) * t26396 - F::cast_from(0.52945875e1_f64) * t26398 + F::cast_from(0.41318e1_f64) * t26280 - F::cast_from(0.123954e2_f64) * t26284 - F::cast_from(0.103295e1_f64) * t26293 + F::cast_from(0.123954e2_f64) * t26296 + F::cast_from(0.309885e1_f64) * t26304 - F::cast_from(0.27545333333333333332e1_f64) * t26311 + F::cast_from(0.166712e1_f64) * t26406 - F::cast_from(0.125034e1_f64) * t26409 - F::cast_from(0.104195e0_f64) * t26412 + F::cast_from(0.250068e1_f64) * t26415;
    t26642
}
