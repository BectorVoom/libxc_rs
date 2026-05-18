//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1251/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1251<F: Float>(t2785: F, t7274: F, t913: F, t7982: F, t8196: F, t2663: F, t276: F, t308: F, t115: F, t282: F, t8206: F, t25797: F, t2674: F, t8134: F) -> (F, F, F, F, F, F) {
    let t25821 = t913 * t7274 * t2785;
    let t25826 = t8196 * t7982;
    let t25834 = F::new(1.0) / t2663 / t308 / t276;
    let t25836 = t282 * t25834 * t115;
    let t25837 = t8206 * t25836;
    let t25843 = t8134 * t25797 * t2674;
    (t25821, t25826, t25834, t25836, t25837, t25843)
}
