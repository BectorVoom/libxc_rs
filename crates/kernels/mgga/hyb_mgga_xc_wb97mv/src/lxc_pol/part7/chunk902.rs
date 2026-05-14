//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 902/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk902<F: Float>(t1226: F, t2003: F, t19: F, t556: F, t92: F, t125: F, t3011: F, t546: F, t222: F, t3: F, t6811: F) -> (F, F, F, F, F) {
    let t8184 = t2003 * t1226;
    let t8185 = t19 * t8184;
    let t8187 = t556 * t92;
    let t8188 = t8187 * t125;
    let t8193 = t546 * t3011 / 32.0;
    let t8195 = t3 * t6811 * t222;
    (t8184, t8185, t8188, t8193, t8195)
}
