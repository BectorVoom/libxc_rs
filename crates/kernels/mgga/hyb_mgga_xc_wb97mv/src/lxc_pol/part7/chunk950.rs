//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 950/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk950<F: Float>(t1357: F, t2300: F, t3413: F, t838: F, t1353: F, t2257: F, t2279: F, t2287: F, t2303: F, t271: F, t3383: F, t3403: F, t6866: F, t820: F, t829: F, t839: F, t848: F, t8933: F, t9014: F, t9023: F, t9050: F, t9053: F, t9056: F, t9061: F) -> (F, F, F) {
    let t9068 = t1357 * t2300;
    let t9071 = t3413 * t838;
    let t9074 = t8933 - 0.310907e-1 * t9014 * t271 - 0.19751673498613801407e-1 * t9023 + 1.0 * t820 * t9050 + 0.5848223622634646207e0 * t839 * t9053 + 2.0 * t9056 * t829 + 1.0 * t3383 * t2279 + 0.32163958997385070134e2 * t9061 * t2287 + 1.0 * t6866 * t1353 + 2.0 * t2257 * t3403 - 0.11696447245269292414e1 * t9068 * t2303 + 0.11696447245269292414e1 * t9071 * t848;
    (t9068, t9071, t9074)
}
