//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1256/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1256<F: Float>(t28342: F, t28372: F, t94229: F, t28461: F, t7904: F, t1014: F, t28528: F, t54162: F, t8147: F, t2237: F, t15815: F, t303: F, t7931: F) -> (F, F, F, F, F, F) {
    let t98515 = t28372 * t28342 * t94229;
    let t98519 = F::new(0.46336805555555555556e-3) * t28461 * t7904;
    let t98522 = t1014 * t28528;
    let t98524 = t54162 * t8147;
    let t98525 = t2237 * t98524;
    let t98528 = t303 * t7931 * t15815;
    (t98515, t98519, t98522, t98524, t98525, t98528)
}
