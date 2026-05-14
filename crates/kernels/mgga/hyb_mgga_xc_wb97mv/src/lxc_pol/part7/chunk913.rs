//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 913/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk913<F: Float>(t1205: F, t1217: F, t1917: F, t1925: F, t1929: F, t1946: F, t1957: F, t3085: F, t3089: F, t3090: F, t3122: F, t615: F, t617: F, t6265: F, t631: F, t72: F, t8344: F, t8357: F, t8360: F, t8363: F, t8421: F, t85: F) -> (F,) {
    let t8424 = 7.0 / 2.0 * t1946 * t3090 - t8357 * t3090 / 2.0 - t8360 * t3090 / 4.0 - t3089 * t8363 - 6.0 * t6265 * t1205 * t1925 + 4.0 * t1929 * t3085 * t615 + 2.0 * t1929 * t1205 * t1917 - t617 * t8344 + 2.0 * t8344 * t85 + 4.0 * t3085 * t631 + 2.0 * t1205 * t1957 + 2.0 * t1917 * t1217 + 4.0 * t615 * t3122 + 2.0 * t72 * t8421;
    (t8424,)
}
