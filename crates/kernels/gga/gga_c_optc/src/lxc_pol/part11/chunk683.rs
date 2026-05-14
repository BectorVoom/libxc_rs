//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 683/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk683<F: Float>(t7523: F, t2595: F, t56: F, t214: F, t136: F, t2548: F, t745: F, t222: F, t224: F) -> (F, F, F, F, F, F) {
    let t7524 = 28.0 / 27.0 * t7523;
    let t7533 = t56 * t2595;
    let t7557 = 1.0/pow_3_2(t214);
    let t7578 = t136 * t2548;
    let t7590 = t745 * t136;
    let t7592 = t222 * t7590 * t224;
    (t7524, t7533, t7557, t7578, t7590, t7592)
}
