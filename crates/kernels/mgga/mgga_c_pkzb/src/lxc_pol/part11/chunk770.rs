//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 770/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk770<F: Float>(t300: F, t3236: F, t3176: F, t68: F, t3174: F, t1238: F, t2402: F, t2099: F, t3201: F, t918: F, t178: F, t3212: F, t915: F) -> (F, F, F, F, F, F, F) {
    let t8264 = t300 * t3236;
    let t8273 = t68 * t3176;
    let t8275 = t3174 * t8273 / 72.0;
    let t8285 = t1238 * t2402;
    let t8315 = t2099 * t3201;
    let t8317 = 0.28582678745379824648e-3 * t918 * t8315;
    let t8318 = t3212 * t178;
    let t8319 = t915 * t8318;
    (t8264, t8273, t8275, t8285, t8315, t8317, t8319)
}
