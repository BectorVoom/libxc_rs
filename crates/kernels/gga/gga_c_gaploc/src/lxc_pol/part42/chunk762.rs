//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 762/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk762<F: Float>(t11576: F, t795: F, t313: F, t1: F, t36610: F, t2021: F, t2089: F, t106: F, t316: F, t11286: F, t501: F, t11401: F, t540: F) -> (F, F, F, F, F, F, F) {
    let t37060 = t795 * t11576;
    let t37061 = t313 * t37060;
    let t37179 = t36610 * t1;
    let t37180 = t2021 * t37179;
    let t37200 = t2089 * t11576;
    let t37218 = t11576 * t1 * t106 * t316;
    let t37275 = t11286 * t501;
    let t37326 = t11401 * t540;
    (t37060, t37061, t37180, t37200, t37218, t37275, t37326)
}
