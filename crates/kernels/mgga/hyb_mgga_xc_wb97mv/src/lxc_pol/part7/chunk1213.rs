//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1213/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1213<F: Float>(t24963: F, t3: F, t10322: F, t10327: F, t10349: F, t1861: F, t21373: F, t21397: F, t24915: F, t24943: F, t24945: F, t24954: F, t24969: F, t24974: F, t29290: F, t3025: F, t3031: F, t3854: F, t3864: F, t571: F, t6155: F, t8228: F) -> (F, F) {
    let t29291 = t24963 * t3;
    let t29320 = 4.0 / 243.0 * t21373 - 16.0 / 729.0 * t24915 - 4.0 / 81.0 * t24943 + 2.0 / 27.0 * t24945 + 2.0 / 81.0 * t24954 - 40.0 / 243.0 * t29290 * t24974 * t29291 + 16.0 / 27.0 * t29290 * t24969 * t29291 + 2.0 / 27.0 * t571 * t3025 * t6155 * t3854 * t1861 - t571 * t3031 * t10349 * t1861 / 9.0 + 4.0 / 9.0 * t571 * t3031 * t10327 * t1861 + 20.0 / 81.0 * t571 * t8228 * t21397 * t3864 * t1861 - 4.0 / 9.0 * t571 * t3025 * t10322 * t1861;
    (t29291, t29320)
}
