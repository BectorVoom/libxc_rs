//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1381/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1381(t10963: f64, t997: f64, t2598: f64, t4305: f64, t1006: f64, t1007: f64, t11068: f64, t11076: f64, t11149: f64, t1434: f64, t21613: f64, t21700: f64, t21721: f64, t2533: f64, t25630: f64, t2572: f64, t2579: f64, t2594: f64, t2602: f64, t29746: f64, t29750: f64, t29792: f64, t29913: f64, t29932: f64, t29945: f64, t29959: f64, t29972: f64, t3560: f64, t3580: f64, t4300: f64, t4311: f64, t4324: f64, t4327: f64, t7099: f64, t9076: f64, t9196: f64, t9199: f64, t9245: f64, t979: f64, t987: f64, t998: f64) -> f64 {
    let t29980 = t10963 * t997;
    let t29985 = t4305 * t2598;
    let t29988 = -4.0_f64 * t9245 * t9076 - 0.11696447245269292414e1_f64 * t29746 * t2579 - t29750 + t29792 + 0.23392894490538584828e1_f64 * t9199 * t3580 + 0.11696447245269292414e1_f64 * t3560 * t9196 - 0.11696447245269292414e1_f64 * t21613 * t4311 + 0.5848223622634646207e0_f64 * t7099 * t4324 + 0.11696447245269292414e1_f64 * t2572 * t11149 + 0.5848223622634646207e0_f64 * t998 * t29913 * t1006 + 0.17315859105681463759e2_f64 * t21721 * t4327 + 0.11696447245269292414e1_f64 * t25630 * t1434 + 2.0_f64 * t2533 * t11068 + 1.0_f64 * t979 * (t29932 + t29945 + t29959 + t29972) * t987 + 0.32163958997385070134e2_f64 * t21700 * t4300 + 0.11696447245269292414e1_f64 * t29980 * t1007 + 0.5848223622634646207e0_f64 * t11076 * t2594 + 0.17315859105681463759e2_f64 * t29985 * t2602;
    t29988
}
