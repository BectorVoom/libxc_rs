//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1250/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1250<F: Float>(t22716: F, t22719: F, t22724: F, t23431: F, t23438: F, t39066: F, t4595: F, t48009: F, t56295: F, t56296: F, t56297: F, t56299: F, t95: F) -> F {
    let t56667 = -t22716 - t22719 + t56295 + t56296 + t23431 - t23438 + F::new(70.0) / F::new(3.0) * t39066 - t56297 + t22724 + F::new(0.93041573165652349788e-1) * t95 * t48009 * t4595 + t56299;
    t56667
}
