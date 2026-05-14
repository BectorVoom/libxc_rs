//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1180/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1180<F: Float>(t26309: F, t26311: F, t26313: F, t26314: F, t26319: F, t26324: F, t26326: F, t26328: F, t26330: F, t26332: F, t26339: F, t26343: F, t26308: F, t1038: F, t531: F, t8597: F) -> (F, F, F) {
    let t26345 = 8.0 / 9.0 * t26309 - 16.0 / 9.0 * t26311 + t26313 + 4.0 / 9.0 * t26314 + 8.0 / 3.0 * t26319 - 8.0 / 9.0 * t26324 - 8.0 / 9.0 * t26326 - 16.0 / 27.0 * t26328 + 16.0 / 9.0 * t26330 + 112.0 / 81.0 * t26332 - 80.0 / 81.0 * t26339 - t26343 / 3.0;
    let t26346 = t26308 + t26345;
    let t26347 = t1038 * t26346;
    let t26351 = t531 * t8597;
    (t26346, t26347, t26351)
}
