//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1372/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1372<F: Float>(t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t1025: F, t11: F, t58295: F) -> (F, F) {
    let t58433 = F::cast_from(0.2585111111111111111e2_f64) * t58397 - F::cast_from(0.46531999999999999999e2_f64) * t58401 - F::cast_from(0.38776666666666666665e1_f64) * t58405 + F::cast_from(0.46532e2_f64) * t58409 + F::cast_from(0.11633e2_f64) * t58412 - F::cast_from(0.12315e-2_f64) * t58415 - F::cast_from(0.14778e-1_f64) * t58418 - F::cast_from(0.12315e-2_f64) * t58421 + F::cast_from(0.29556e-1_f64) * t58424 - F::cast_from(0.12771111111111111111e-2_f64) * t58428 - F::cast_from(0.57446913580246913579e1_f64) * t58431;
    let t58435 = t11 * t1025 * t58295;
    (t58433, t58435)
}
