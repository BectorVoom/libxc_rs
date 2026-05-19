//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1316/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1316<F: Float>(t24776: F, t30270: F, t39413: F, t39418: F, t49240: F, t49242: F, t49393: F, t49395: F, t56969: F, t57027: F, t57037: F, t57041: F) -> F {
    let t57513 = -F::cast_from(0.34246666666666666665e-1_f64) * t57027 - F::cast_from(0.11415555555555555555e0_f64) * t56969 - F::cast_from(0.3044148148148148148e-1_f64) * t39413 + F::cast_from(0.9132444444444444444e-1_f64) * t39418 + t24776 + F::cast_from(0.4566222222222222222e-1_f64) * t49240 - F::cast_from(0.13698666666666666667e0_f64) * t49242 + F::cast_from(0.22831111111111111111e-1_f64) * t49393 + F::cast_from(0.25367901234567901233e-1_f64) * t49395 + F::cast_from(0.71030123456790123454e-1_f64) * t30270 + F::cast_from(0.2283111111111111111e0_f64) * t57037 - F::cast_from(0.50735802469135802467e-1_f64) * t57041;
    t57513
}
