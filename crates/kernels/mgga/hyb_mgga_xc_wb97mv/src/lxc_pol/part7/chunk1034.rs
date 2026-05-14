//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1034/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1034<F: Float>(t43: F, t10389: F, t10432: F, t10434: F, t10511: F, t1205: F, t1220: F, t3085: F, t3125: F, t3926: F, t3962: F, t615: F, t634: F, t72: F, t88: F, t29: F, t125: F) -> (F, F, F) {
    let t44 = 0.135e1 <= t43;
    let t10515 = piecewise3(t44, t10389 + t10432, -8.0 / 3.0 * t10434 * t88 - 8.0 / 3.0 * t3926 * t634 - 16.0 / 3.0 * t3085 * t1220 - 16.0 / 3.0 * t1205 * t3125 - 8.0 / 3.0 * t615 * t3962 - 8.0 / 3.0 * t72 * t10511);
    let t10516 = t29 * t10515;
    let t10517 = t10516 * t125;
    (t10515, t10516, t10517)
}
