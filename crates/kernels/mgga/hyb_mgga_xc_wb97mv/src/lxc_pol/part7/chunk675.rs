//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 675/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk675<F: Float>(t3439: F, t846: F, t1353: F, t1365: F, t2257: F, t2262: F, t2284: F, t2296: F, t2301: F, t2323: F, t271: F, t3332: F, t3335: F, t3337: F, t3340: F, t3372: F, t3376: F, t3380: F, t3383: F, t3388: F, t3403: F, t3407: F, t3414: F, t3416: F, t3421: F, t3436: F, t820: F, t829: F, t839: F, t848: F) -> (F, F) {
    let t3440 = t3439 * t846;
    let t3443 = -0.310907e-1 * t3380 * t271 + 1.0 * t3383 * t829 + 1.0 * t2257 * t1353 - 2.0 * t2262 * t3388 + 1.0 * t820 * t3403 + 0.32163958997385070134e2 * t2284 * t3407 + t3332 - t3335 - t3337 + t3340 - t3372 - t3376 - 0.19751673498613801407e-1 * t3414 + 0.5848223622634646207e0 * t3416 * t848 + 0.5848223622634646207e0 * t2296 * t1365 - 0.11696447245269292414e1 * t2301 * t3421 + 0.5848223622634646207e0 * t839 * t3436 + 0.17315859105681463759e2 * t2323 * t3440;
    (t3440, t3443)
}
