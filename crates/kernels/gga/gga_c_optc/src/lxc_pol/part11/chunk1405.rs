//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1405/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1405<F: Float>(t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t58435: F, t58754: F) -> F {
    let t59132 = -F::cast_from(0.82156666666666666667e-1_f64) * t58415 - F::cast_from(0.98587999999999999998e0_f64) * t58418 - F::cast_from(0.82156666666666666668e-1_f64) * t58421 + F::new(0.197176e1) * t58424 - F::cast_from(0.85199506172839506175e-1_f64) * t58428 - F::cast_from(0.88582716049382716048e0_f64) * t58431 - F::cast_from(0.29896666666666666667e0_f64) * t58435 + F::cast_from(0.97370864197530864196e-1_f64) * t52591 - F::cast_from(0.43816888888888888888e0_f64) * t52593 + F::cast_from(0.13145066666666666666e1_f64) * t52596 + F::cast_from(0.21908444444444444444e0_f64) * t52601 + F::cast_from(0.79724444444444444444e0_f64) * t52446 - F::cast_from(0.23917333333333333333e1_f64) * t52452 + F::new(0.1898925e1) * t58754;
    t59132
}
