//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1026/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1026<F: Float>(t3894: F, t69: F, t3877: F, t6238: F, t1911: F, t587: F, t1852: F, t3882: F, t3864: F, t6147: F, t544: F, t8228: F, t6155: F, t3025: F, t3: F, t3026: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10303 = t69 * t3894;
    let t10306 = t6238 * t3877;
    let t10311 = t1911 * t3894;
    let t10314 = t587 * t3877;
    let t10320 = t1852 * t3882;
    let t10322 = t6147 * t3864;
    let t10324 = t8228 * t10322 * t544;
    let t10327 = t6155 * t3864;
    let t10329 = t3025 * t10327 * t544;
    let t10333 = t3025 * t3026 * t3;
    (t10303, t10306, t10311, t10314, t10320, t10322, t10324, t10327, t10329, t10333)
}
