//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1006/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1006<F: Float>(t8098: F, t890: F, t3113: F, t881: F, t1201: F, t2317: F, t1209: F, t2298: F, t2313: F, t2321: F, t3116: F, t6294: F, t7924: F, t7926: F, t8006: F, t8011: F, t8041: F, t8068: F, t8071: F, t863: F, t882: F, t891: F) -> (F, F, F, F) {
    let t8099 = t8098 * t890;
    let t8102 = t3113 * t881;
    let t8107 = t1201 * t2317;
    let t8112 = -F::cast_from(0.19751673498613801407e-1_f64) * t8041 - t7924 - t7926 - t8006 + F::cast_from(1.0_f64) * t863 * t8068 - F::cast_from(0.11696447245269292414e1_f64) * t8071 * t2298 + t8011 + F::cast_from(0.5848223622634646207e0_f64) * t882 * t8099 + F::cast_from(0.11696447245269292414e1_f64) * t8102 * t891 + F::cast_from(0.5848223622634646207e0_f64) * t3116 * t2313 + F::cast_from(0.17315859105681463759e2_f64) * t8107 * t2321 + F::cast_from(0.5848223622634646207e0_f64) * t6294 * t1209;
    (t8099, t8102, t8107, t8112)
}
