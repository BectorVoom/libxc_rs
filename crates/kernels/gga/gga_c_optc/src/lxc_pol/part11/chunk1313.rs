//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1313/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1313<F: Float>(t39545: F, t39560: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t57037: F, t57041: F, t57044: F, t57048: F) -> F {
    let t57432 = F::cast_from(0.76514814814814814814e0_f64) * t49395 + F::new(0.250068e1) * t57012 + F::new(0.62517e0) * t57016 - F::new(0.104195e0) * t57020 - F::new(0.123954e2) * t57024 - F::new(0.103295e1) * t57027 - F::new(0.125034e1) * t57030 + F::cast_from(0.55570666666666666666e0_f64) * t57034 + F::cast_from(0.68863333333333333334e1_f64) * t57037 - F::cast_from(0.15302962962962962963e1_f64) * t57041 - F::cast_from(0.10805407407407407407e0_f64) * t57044 - F::new(0.104195e0) * t57048 - F::cast_from(0.23154444444444444445e0_f64) * t39545 - F::cast_from(0.69463333333333333334e0_f64) * t39560;
    t57432
}
