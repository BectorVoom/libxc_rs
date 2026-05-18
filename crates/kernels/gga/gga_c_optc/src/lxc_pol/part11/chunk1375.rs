//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1375/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1375<F: Float>(t27950: F, t27951: F, t52389: F, t52391: F, t52393: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F) -> F {
    let t58498 = F::new(0.96922222222222222224e3) * t52389 + F::new(0.58153333333333333332e4) * t52391 + F::new(0.10769135802469135803e4) * t52393 - F::new(0.38768888888888888889e4) * t52395 + t27950 + t27951 - F::new(0.4846111111111111111e4) * t58375 + F::new(0.17445999999999999999e5) * t58378 - F::new(0.19384444444444444444e4) * t58381 - F::new(26169.0) * t58384 + F::new(0.41955555555555555555e3) * t58388 + F::new(0.47199999999999999999e3) * t58392;
    t58498
}
