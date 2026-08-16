//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1298/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1298<F: Float>(t39545: F, t39560: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t57037: F, t57041: F, t57044: F, t57048: F) -> F {
    let t57164 = F::cast_from(0.44729629629629629629e0_f64) * t49395 + F::cast_from(0.198684e1_f64) * t57012 + F::cast_from(0.49671e0_f64) * t57016 - F::cast_from(0.82785e-1_f64) * t57020 - F::cast_from(0.72462e1_f64) * t57024 - F::cast_from(0.60384999999999999999e0_f64) * t57027 - F::cast_from(0.99342e0_f64) * t57030 + F::cast_from(0.44152e0_f64) * t57034 + F::cast_from(0.40256666666666666666e1_f64) * t57037 - F::cast_from(0.89459259259259259259e0_f64) * t57041 - F::cast_from(0.8585111111111111111e-1_f64) * t57044 - F::cast_from(0.82785e-1_f64) * t57048 - F::cast_from(0.18396666666666666667e0_f64) * t39545 - F::cast_from(0.5519e0_f64) * t39560;
    t57164
}
