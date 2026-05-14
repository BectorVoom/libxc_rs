//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 681/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk681<F: Float>(t1022: F, t7275: F, t1: F, t32364: F, t787: F, t10938: F, t2021: F, t33137: F, t10007: F, t10627: F, t32435: F, t739: F, t106: F, t10667: F, t316: F, t11000: F, t783: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33360 = t7275 * t1022;
    let t33399 = t787 * t32364 * t1;
    let t33565 = t2021 * t10938;
    let t33575 = t33137 * t1;
    let t33576 = t2021 * t33575;
    let t33601 = t10007 * t10627;
    let t33680 = t739 * t32435;
    let t33725 = t10667 * t1 * t106 * t316;
    let t33778 = t11000 * t783;
    (t33360, t33399, t33565, t33575, t33576, t33601, t33680, t33725, t33778)
}
