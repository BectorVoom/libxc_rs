//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1381/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1381(t26836: f64, t33724: f64, t43414: f64, t43503: f64, t43508: f64, t52389: f64, t52391: f64, t52393: f64, t52395: f64, t52446: f64, t52452: f64, t58435: f64) -> f64 {
    let t58626 = -0.17808333333333333333e-1_f64 * t58435 + 0.23744444444444444444e-1_f64 * t52389 + 0.14246666666666666667e0_f64 * t52391 - 0.47488888888888888888e-1_f64 * t43503 + 0.94977777777777777776e-1_f64 * t43508 + 0.47488888888888888888e-1_f64 * t52446 - 0.14246666666666666667e0_f64 * t52452 + 0.73871604938271604937e-1_f64 * t33724 + t26836 + 0.26382716049382716049e-1_f64 * t52393 - 0.94977777777777777776e-1_f64 * t52395 - 0.31659259259259259258e-1_f64 * t43414;
    t58626
}
