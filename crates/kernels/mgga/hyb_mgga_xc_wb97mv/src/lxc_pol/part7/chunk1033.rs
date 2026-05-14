//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1033/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1033<F: Float>(t10434: F, t10448: F, t10457: F, t10508: F, t1205: F, t1217: F, t1929: F, t3085: F, t3089: F, t3104: F, t3122: F, t3926: F, t3931: F, t3948: F, t3959: F, t615: F, t617: F, t6265: F, t631: F, t72: F, t8363: F, t8385: F, t85: F) -> (F,) {
    let t10511 = 7.0 / 2.0 * t3948 * t3104 - t8385 * t8363 - t10448 * t3104 / 4.0 - 6.0 * t6265 * t3931 * t615 + 4.0 * t1929 * t1205 * t3085 - t3089 * t10457 / 2.0 + 2.0 * t1929 * t3926 * t615 - t617 * t10434 + 2.0 * t10434 * t85 + 2.0 * t3926 * t631 + 4.0 * t3085 * t1217 + 4.0 * t1205 * t3122 + 2.0 * t615 * t3959 + 2.0 * t72 * t10508;
    (t10511,)
}
