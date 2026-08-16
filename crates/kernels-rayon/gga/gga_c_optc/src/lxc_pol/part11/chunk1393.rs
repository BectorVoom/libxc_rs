//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1393/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1393(t26780: f64, t33724: f64, t43414: f64, t43503: f64, t43508: f64, t52389: f64, t52391: f64, t52393: f64, t52395: f64, t52446: f64, t52452: f64, t58435: f64) -> f64 {
    let t58860 = -0.92708333333333333333e-2_f64 * t58435 + 0.12361111111111111111e-1_f64 * t52389 + 0.74166666666666666668e-1_f64 * t52391 - 0.24722222222222222222e-1_f64 * t43503 + 0.49444444444444444445e-1_f64 * t43508 + 0.24722222222222222222e-1_f64 * t52446 - 0.74166666666666666668e-1_f64 * t52452 + 0.38456790123456790123e-1_f64 * t33724 + t26780 + 0.13734567901234567901e-1_f64 * t52393 - 0.49444444444444444444e-1_f64 * t52395 - 0.16481481481481481482e-1_f64 * t43414;
    t58860
}
