//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1065/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1065<F: Float>(t2529: F, t7604: F, t838: F, t845: F, t23: F, t2326: F, t2328: F, t2331: F, t7224: F, t7253: F, t8: F, t191: F, t2436: F, t7257: F, t2270: F, t7221: F, t7222: F) -> (F, F, F, F) {
    let t24141 = 0.46785787179641632568e1 * t845 * t2529 * t7604 * t838;
    let t24145 = t2326 * t2328 * t2331 * t23;
    let t24146 = t24145 * t7224;
    let t24148 = t8 * t7253;
    let t24149 = t24148 * t191;
    let t24151 = t24149 * t2436 * t7257;
    let t24155 = t7221 * t7222 * t2270;
    (t24141, t24146, t24151, t24155)
}
