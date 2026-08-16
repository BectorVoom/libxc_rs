//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1416/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1416(t52446: f64, t52452: f64, t52591: f64, t52593: f64, t52596: f64, t52601: f64, t58415: f64, t58418: f64, t58421: f64, t58424: f64, t58428: f64, t58431: f64, t58435: f64, t58754: f64) -> f64 {
    let t59310 = -0.104195e0_f64 * t58415 - 0.125034e1_f64 * t58418 - 0.104195e0_f64 * t58421 + 0.250068e1_f64 * t58424 - 0.10805407407407407407e0_f64 * t58428 - 0.15302962962962962963e1_f64 * t58431 - 0.516475e0_f64 * t58435 + 0.12349037037037037037e0_f64 * t52591 - 0.55570666666666666668e0_f64 * t52593 + 0.166712e1_f64 * t52596 + 0.27785333333333333333e0_f64 * t52601 + 0.13772666666666666667e1_f64 * t52446 - 0.41318e1_f64 * t52452 + 0.3529725e1_f64 * t58754;
    t59310
}
