//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1381/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1381<F: Float>(t26836: F, t33724: F, t43414: F, t43503: F, t43508: F, t52389: F, t52391: F, t52393: F, t52395: F, t52446: F, t52452: F, t58435: F) -> F {
    let t58626 = -F::cast_from(0.17808333333333333333e-1_f64) * t58435 + F::cast_from(0.23744444444444444444e-1_f64) * t52389 + F::cast_from(0.14246666666666666667e0_f64) * t52391 - F::cast_from(0.47488888888888888888e-1_f64) * t43503 + F::cast_from(0.94977777777777777776e-1_f64) * t43508 + F::cast_from(0.47488888888888888888e-1_f64) * t52446 - F::cast_from(0.14246666666666666667e0_f64) * t52452 + F::cast_from(0.73871604938271604937e-1_f64) * t33724 + t26836 + F::cast_from(0.26382716049382716049e-1_f64) * t52393 - F::cast_from(0.94977777777777777776e-1_f64) * t52395 - F::cast_from(0.31659259259259259258e-1_f64) * t43414;
    t58626
}
