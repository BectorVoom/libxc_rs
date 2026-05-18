//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1006/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1006<F: Float>(t6: F, t9771: F, t2020: F, t6892: F, t2029: F, t6875: F, t39: F, t55: F, t59: F, t87: F, t1759: F, t1784: F, t1790: F) -> (F, F, F, F, F) {
    let t22166 = t9771 * t6;
    let t22242 = t2020 * t6892;
    let t22265 = t6875 * t2029;
    let t22274 = F::new(24.0) * t39 * t55 * t59 * t87;
    let t22277 = F::new(36.0) * t1790 * t1759 * t1784;
    (t22166, t22242, t22265, t22274, t22277)
}
