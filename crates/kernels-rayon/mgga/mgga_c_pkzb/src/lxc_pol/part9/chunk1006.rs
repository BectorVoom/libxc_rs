//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1006/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1006(t8098: f64, t890: f64, t3113: f64, t881: f64, t1201: f64, t2317: f64, t1209: f64, t2298: f64, t2313: f64, t2321: f64, t3116: f64, t6294: f64, t7924: f64, t7926: f64, t8006: f64, t8011: f64, t8041: f64, t8068: f64, t8071: f64, t863: f64, t882: f64, t891: f64) -> (f64, f64, f64, f64) {
    let t8099 = t8098 * t890;
    let t8102 = t3113 * t881;
    let t8107 = t1201 * t2317;
    let t8112 = -0.19751673498613801407e-1_f64 * t8041 - t7924 - t7926 - t8006 + 1.0_f64 * t863 * t8068 - 0.11696447245269292414e1_f64 * t8071 * t2298 + t8011 + 0.5848223622634646207e0_f64 * t882 * t8099 + 0.11696447245269292414e1_f64 * t8102 * t891 + 0.5848223622634646207e0_f64 * t3116 * t2313 + 0.17315859105681463759e2_f64 * t8107 * t2321 + 0.5848223622634646207e0_f64 * t6294 * t1209;
    (t8099, t8102, t8107, t8112)
}
