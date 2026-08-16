//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1395/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1395<F: Float>(t17356: F, t4299: F, t4300: F, t277: F, t2911: F, t3245: F, t34301: F, t34309: F, t4281: F, t4282: F, t4289: F, t4290: F, t4297: F, t53769: F, t53776: F, t58827: F, t58834: F, t58836: F, t58864: F, t58865: F, t95: F) -> F {
    let t58875 = t4299 * t4300 * t17356;
    let t58878 = -F::cast_from(2464.0_f64) / F::cast_from(81.0_f64) * t53769 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t34301 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t34309 - F::cast_from(0.77534644304710291488e-2_f64) * t95 * t277 * t58827 * t2911 + F::cast_from(80000.0_f64) / F::cast_from(81.0_f64) * t53776 + t58834 + t58836 + t58864 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4281 * t3245 * t4282 * t58865 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4281 * t4289 * t4290 * t58865 + F::cast_from(200.0_f64) / F::cast_from(81.0_f64) * t4297 * t58875;
    t58878
}
