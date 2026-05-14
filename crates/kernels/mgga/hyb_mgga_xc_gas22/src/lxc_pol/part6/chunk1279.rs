//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1279/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1279<F: Float>(t10963: F, t997: F, t2598: F, t4305: F, t1006: F, t1007: F, t11068: F, t11076: F, t11149: F, t1434: F, t21613: F, t21700: F, t21721: F, t2533: F, t25630: F, t2572: F, t2579: F, t2594: F, t2602: F, t29746: F, t29750: F, t29792: F, t29913: F, t29932: F, t29945: F, t29959: F, t29972: F, t3560: F, t3580: F, t4300: F, t4311: F, t4324: F, t4327: F, t7099: F, t9076: F, t9196: F, t9199: F, t9245: F, t979: F, t987: F, t998: F) -> (F,) {
    let t29980 = t10963 * t997;
    let t29985 = t4305 * t2598;
    let t29988 = -4.0 * t9245 * t9076 - 0.11696447245269292414e1 * t29746 * t2579 - t29750 + t29792 + 0.23392894490538584828e1 * t9199 * t3580 + 0.11696447245269292414e1 * t3560 * t9196 - 0.11696447245269292414e1 * t21613 * t4311 + 0.5848223622634646207e0 * t7099 * t4324 + 0.11696447245269292414e1 * t2572 * t11149 + 0.5848223622634646207e0 * t998 * t29913 * t1006 + 0.17315859105681463759e2 * t21721 * t4327 + 0.11696447245269292414e1 * t25630 * t1434 + 2.0 * t2533 * t11068 + 1.0 * t979 * (t29932 + t29945 + t29959 + t29972) * t987 + 0.32163958997385070134e2 * t21700 * t4300 + 0.11696447245269292414e1 * t29980 * t1007 + 0.5848223622634646207e0 * t11076 * t2594 + 0.17315859105681463759e2 * t29985 * t2602;
    (t29988,)
}
