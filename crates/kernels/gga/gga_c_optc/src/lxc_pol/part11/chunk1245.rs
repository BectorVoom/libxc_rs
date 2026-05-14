//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1245/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1245<F: Float>(t26780: F, t33724: F, t43414: F, t43503: F, t43508: F, t52389: F, t52391: F, t52393: F, t52395: F, t52446: F, t52452: F, t58435: F, t415: F, t58848: F, t241: F, t1239: F, t16241: F) -> (F, F, F) {
    let t58860 = -0.92708333333333333333e-2 * t58435 + 0.12361111111111111111e-1 * t52389 + 0.74166666666666666668e-1 * t52391 - 0.24722222222222222222e-1 * t43503 + 0.49444444444444444445e-1 * t43508 + 0.24722222222222222222e-1 * t52446 - 0.74166666666666666668e-1 * t52452 + 0.38456790123456790123e-1 * t33724 + t26780 + 0.13734567901234567901e-1 * t52393 - 0.49444444444444444444e-1 * t52395 - 0.16481481481481481482e-1 * t43414;
    let t58862 = (t58848 + t58860) * t415;
    let t58864 = 0.19751789702565206229e-1 * t241 * t58862;
    let t58865 = t16241 * t1239;
    (t58862, t58864, t58865)
}
