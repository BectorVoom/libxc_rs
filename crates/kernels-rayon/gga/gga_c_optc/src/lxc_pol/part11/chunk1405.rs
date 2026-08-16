//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1405/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1405(t52446: f64, t52452: f64, t52591: f64, t52593: f64, t52596: f64, t52601: f64, t58415: f64, t58418: f64, t58421: f64, t58424: f64, t58428: f64, t58431: f64, t58435: f64, t58754: f64) -> f64 {
    let t59132 = -0.82156666666666666667e-1_f64 * t58415 - 0.98587999999999999998e0_f64 * t58418 - 0.82156666666666666668e-1_f64 * t58421 + 0.197176e1_f64 * t58424 - 0.85199506172839506175e-1_f64 * t58428 - 0.88582716049382716048e0_f64 * t58431 - 0.29896666666666666667e0_f64 * t58435 + 0.97370864197530864196e-1_f64 * t52591 - 0.43816888888888888888e0_f64 * t52593 + 0.13145066666666666666e1_f64 * t52596 + 0.21908444444444444444e0_f64 * t52601 + 0.79724444444444444444e0_f64 * t52446 - 0.23917333333333333333e1_f64 * t52452 + 0.1898925e1_f64 * t58754;
    t59132
}
