//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1114/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1114<F: Float>(t3235: F, t3237: F, t5939: F, t179: F, t3026: F, t404: F, t6380: F, t1184: F, t2240: F, t237: F, t6323: F, t1208: F) -> (F, F, F, F, F) {
    let t22469 = t3235 * t5939 * t3237;
    let t22474 = t404 * t179 * t6380 * t3026;
    let t22475 = F::cast_from(0.28582678745379824648e-3_f64) * t22474;
    let t22500 = t2240 * t1184;
    let t22503 = t237 * t6323;
    let t22561 = t6323 * t1208;
    (t22469, t22475, t22500, t22503, t22561)
}
