//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 673/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk673<F: Float>(t3443: F, t3444: F, t1379: F, t2322: F, t260: F, t3315: F, t3318: F, t3320: F, t3323: F, t3355: F, t3359: F, t3397: F, t3426: F, t3430: F, t3436: F, t3440: F, t856: F, t858: F) -> (F, F) {
    let t3445 = t3443 * t3444;
    let t3448 = -t3315 + t3318 + t3320 - t3323 + t3355 + t3359 + t260 * t3426 + 0.19751673498613801407e-1 * t260 * t3397 - 0.5848223622634646207e0 * t3430 * t858 - 0.5848223622634646207e0 * t2322 * t1379 + 0.11696447245269292414e1 * t856 * t3436 - 0.5848223622634646207e0 * t856 * t3440 - 0.17315859105681463759e2 * t856 * t3445;
    (t3445, t3448)
}
