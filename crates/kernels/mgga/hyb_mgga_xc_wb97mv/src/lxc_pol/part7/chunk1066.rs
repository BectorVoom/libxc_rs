//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1066/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1066<F: Float>(t11096: F, t847: F, t4197: F, t819: F, t4224: F, t838: F, t11068: F, t1353: F, t1365: F, t2257: F, t2296: F, t3383: F, t3403: F, t4203: F, t4216: F, t4219: F, t4243: F, t4246: F, t6871: F, t6923: F, t6977: F, t820: F, t829: F, t839: F, t848: F, t9056: F, t9071: F) -> (F, F, F, F) {
    let t11097 = t11096 * t847;
    let t11100 = t4197 * t819;
    let t11113 = t4224 * t838;
    let t11118 = 1.0 * t820 * t11068 + 0.32163958997385070134e2 * t6871 * t4219 + 0.5848223622634646207e0 * t2296 * t4243 + 0.5848223622634646207e0 * t839 * t11097 + 1.0 * t11100 * t829 + 2.0 * t9056 * t1353 + 2.0 * t3383 * t3403 - 2.0 * t6923 * t4203 + 1.0 * t2257 * t4216 + 0.17315859105681463759e2 * t6977 * t4246 + 0.5848223622634646207e0 * t11113 * t848 + 0.11696447245269292414e1 * t9071 * t1365;
    (t11097, t11100, t11113, t11118)
}
