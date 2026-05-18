//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1022/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1022<F: Float>(t11286: F, t871: F, t1185: F, t3765: F, t2197: F, t1184: F, t9859: F, t2240: F, t1197: F, t3792: F, t10012: F, t1196: F) -> (F, F, F, F, F, F, F) {
    let t11287 = t11286 * t871;
    let t11290 = t1185 * t3765;
    let t11292 = F::new(6.0) * t2197 * t11290;
    let t11293 = t9859 * t1184;
    let t11295 = F::new(0.48245938496077605201e2) * t2240 * t11293;
    let t11296 = t1197 * t3792;
    let t11299 = t10012 * t1196;
    (t11287, t11290, t11292, t11293, t11295, t11296, t11299)
}
