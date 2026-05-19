//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1360/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1360<F: Float>(t27866: F, t27867: F, t52389: F, t52391: F, t52393: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F) -> F {
    let t58394 = F::cast_from(0.25851111111111111111e1_f64) * t52389 + F::cast_from(0.15510666666666666667e2_f64) * t52391 + F::cast_from(0.28723456790123456789e1_f64) * t52393 - F::cast_from(0.10340444444444444444e2_f64) * t52395 + t27866 + t27867 - F::cast_from(0.12925555555555555555e2_f64) * t58375 + F::cast_from(0.46531999999999999998e2_f64) * t58378 - F::cast_from(0.5170222222222222222e1_f64) * t58381 - F::new(0.69798e2) * t58384 + F::new(0.6568e-2) * t58388 + F::new(0.7389e-2) * t58392;
    t58394
}
