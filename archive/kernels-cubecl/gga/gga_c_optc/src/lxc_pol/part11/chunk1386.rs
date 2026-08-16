//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1386/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1386<F: Float>(t58348: F, t58375: F, t58378: F, t58381: F, t58384: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F, t58431: F, t26313: F, t33724: F, t43414: F, t43503: F, t43508: F, t52389: F, t52391: F, t52393: F, t52395: F, t52446: F, t52452: F, t58435: F) -> (F, F) {
    let t58740 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t58397 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t58375 - F::cast_from(8.0_f64) * t58401 + F::cast_from(8.0_f64) * t58378 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t58405 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t58381 + F::cast_from(8.0_f64) * t58409 - F::cast_from(12.0_f64) * t58384 + F::cast_from(2.0_f64) * t58412 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t58348 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t58431;
    let t58752 = -t58435 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t52389 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t52391 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43503 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43508 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t52446 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t52452 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t33724 + t26313 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t52393 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t52395 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t43414;
    (t58740, t58752)
}
