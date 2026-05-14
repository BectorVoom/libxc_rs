//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1019/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1019<F: Float>(t179: F, t3026: F, t404: F, t6380: F, t1184: F, t2240: F, t237: F, t6323: F, t1208: F, t2295: F, t3113: F, t1201: F, t6121: F, t1196: F, t2279: F, t6313: F) -> (F, F, F, F, F, F, F, F) {
    let t22474 = t404 * t179 * t6380 * t3026;
    let t22475 = 0.28582678745379824648e-3 * t22474;
    let t22500 = t2240 * t1184;
    let t22503 = t237 * t6323;
    let t22561 = t6323 * t1208;
    let t22564 = t3113 * t2295;
    let t22567 = t1201 * t6121;
    let t22575 = t2279 * t1196;
    let t22639 = t6313 * t1196;
    (t22475, t22500, t22503, t22561, t22564, t22567, t22575, t22639)
}
