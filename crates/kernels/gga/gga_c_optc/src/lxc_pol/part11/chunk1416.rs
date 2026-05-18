//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1416/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1416<F: Float>(t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t58415: F, t58418: F, t58421: F, t58424: F, t58428: F, t58431: F, t58435: F, t58754: F) -> F {
    let t59310 = -F::new(0.104195e0) * t58415 - F::new(0.125034e1) * t58418 - F::new(0.104195e0) * t58421 + F::new(0.250068e1) * t58424 - F::new(0.10805407407407407407e0) * t58428 - F::new(0.15302962962962962963e1) * t58431 - F::new(0.516475e0) * t58435 + F::new(0.12349037037037037037e0) * t52591 - F::new(0.55570666666666666668e0) * t52593 + F::new(0.166712e1) * t52596 + F::new(0.27785333333333333333e0) * t52601 + F::new(0.13772666666666666667e1) * t52446 - F::new(0.41318e1) * t52452 + F::new(0.3529725e1) * t58754;
    t59310
}
