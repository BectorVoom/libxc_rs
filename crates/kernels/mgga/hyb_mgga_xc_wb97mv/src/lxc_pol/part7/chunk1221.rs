//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1221/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1221<F: Float>(t10434: F, t10457: F, t10478: F, t10508: F, t1205: F, t1217: F, t1917: F, t1925: F, t1946: F, t1957: F, t21635: F, t25207: F, t29466: F, t29528: F, t29585: F, t29622: F, t3085: F, t3093: F, t3110: F, t3122: F, t3926: F, t3931: F, t3959: F, t615: F, t6265: F, t631: F, t72: F, t8344: F, t8386: F, t8421: F, t85: F) -> (F,) {
    let t29630 = -24.0 * t10478 * t8386 + 24.0 * t21635 * t3931 * t1925 + 7.0 / 2.0 * t1946 * t10457 - 6.0 * t6265 * t3926 * t1925 + 14.0 * t3110 * t29528 - t25207 * t29528 - 24.0 * t6265 * t3093 * t3085 + 2.0 * t3926 * t1957 + 4.0 * t8344 * t1217 + 8.0 * t3085 * t3122 + 4.0 * t1205 * t8421 + 2.0 * t29466 * t85 + 4.0 * t10434 * t631 + 2.0 * t72 * (t29585 + t29622) + 2.0 * t1917 * t3959 + 4.0 * t615 * t10508;
    (t29630,)
}
