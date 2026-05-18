//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1385/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1385<F: Float>(t26496: F, t26497: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F) -> F {
    let t58714 = -F::new(0.16102666666666666667e1) * t52395 + t26496 + t26497 - F::new(0.20128333333333333334e1) * t58375 + F::new(0.72462e1) * t58378 - F::new(0.80513333333333333332e0) * t58381 - F::new(0.108693e2) * t58384 + F::new(0.44152e0) * t58388 + F::new(0.49671e0) * t58392 + F::new(0.40256666666666666666e1) * t58397 - F::new(0.72462e1) * t58401 - F::new(0.60384999999999999999e0) * t58405 + F::new(0.72462e1) * t58409 + F::new(0.181155e1) * t58412;
    t58714
}
