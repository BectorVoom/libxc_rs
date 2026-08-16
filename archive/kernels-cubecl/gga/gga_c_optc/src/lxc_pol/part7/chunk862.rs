//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 862/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk862<F: Float>(t2760: F, t888: F, t2758: F, t2753: F, t2751: F, t140: F, t2665: F, t883: F, t2661: F, t2748: F, t7878: F, t942: F) -> (F, F, F, F, F, F, F) {
    let t8243 = t888 * t2760;
    let t8244 = t2758 * t8243;
    let t8246 = t888 * t2753;
    let t8247 = t2751 * t8246;
    let t8250 = t883 * t2665 * t140;
    let t8251 = t2661 * t8250;
    let t8254 = t2748 * t8250;
    let t8257 = t7878 * t942;
    (t8243, t8244, t8246, t8247, t8251, t8254, t8257)
}
