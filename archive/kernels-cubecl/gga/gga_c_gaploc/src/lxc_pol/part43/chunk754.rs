//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 754/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk754<F: Float>(t3234: F, t701: F, t2610: F, t22542: F, t822: F, t2021: F, t6109: F, t899: F, t1858: F, t3209: F, t107: F, t408: F) -> (F, F, F, F, F, F) {
    let t28236 = t3234 * t701;
    let t28302 = t2610 * t28236;
    let t28309 = t822 * t22542;
    let t28412 = t2021 * t6109 * t899;
    let t28431 = t1858 * t3209;
    let t28438 = t107 * t408;
    (t28236, t28302, t28309, t28412, t28431, t28438)
}
