//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 377/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk377<F: Float>(t1329: F, t1343: F, t1345: F, t1353: F, t1358: F, t1365: F, t1373: F, t260: F, t271: F, t820: F, t839: F, t855: F) -> (F,) {
    let t1376 = -t1329 + t1343 + t260 * (-0.310907e-1 * t1345 * t271 + 1.0 * t820 * t1353 + t1329 - t1343 - 0.19751673498613801407e-1 * t1358 + 0.5848223622634646207e0 * t839 * t1365) + 0.19751673498613801407e-1 * t260 * t1358 - 0.5848223622634646207e0 * t855 * t1373;
    (t1376,)
}
