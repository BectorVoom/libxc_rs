//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1330/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1330<F: Float>(t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26324: F, t26394: F, t26396: F, t26398: F, t26406: F, t26409: F, t26412: F, t26415: F) -> F {
    let t26539 = -F::cast_from(0.80513333333333333332e0_f64) * t26324 - F::cast_from(0.132456e1_f64) * t26394 + F::cast_from(0.22076e0_f64) * t26396 - F::cast_from(0.3883875e1_f64) * t26398 + F::cast_from(0.24154e1_f64) * t26280 - F::cast_from(0.72462e1_f64) * t26284 - F::cast_from(0.60384999999999999999e0_f64) * t26293 + F::cast_from(0.72462e1_f64) * t26296 + F::cast_from(0.181155e1_f64) * t26304 - F::cast_from(0.16102666666666666667e1_f64) * t26311 + F::cast_from(0.132456e1_f64) * t26406 - F::cast_from(0.99342e0_f64) * t26409 - F::cast_from(0.82785e-1_f64) * t26412 + F::cast_from(0.198684e1_f64) * t26415;
    t26539
}
