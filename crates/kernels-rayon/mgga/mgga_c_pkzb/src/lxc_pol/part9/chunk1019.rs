//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1019/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1019(t3160: f64, t6337: f64, t898: f64, t2313: f64, t3152: f64, t8098: f64, t881: f64, t890: f64, t2298: f64, t2328: f64, t3153: f64, t8147: f64, t8185: f64, t8187: f64, t8191: f64, t8194: f64, t8197: f64, t8201: f64, t8204: f64, t8208: f64, t8216: f64, t8218: f64, t8221: f64, t8237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8293 = t3160 * t6337;
    let t8295 = 0.17315859105681463759e2_f64 * t898 * t8293;
    let t8296 = t3152 * t2313;
    let t8298 = 0.11696447245269292414e1_f64 * t898 * t8296;
    let t8300 = t881 * t8098 * t890;
    let t8302 = 0.5848223622634646207e0_f64 * t898 * t8300;
    let t8303 = t3160 * t2298;
    let t8305 = 0.35089341735807877242e1_f64 * t898 * t8303;
    let t8307 = 0.23392894490538584828e1_f64 * t2328 * t3153;
    let t8308 = -t8295 + t8147 - t8185 + t8187 - t8191 - t8194 - t8197 + t8201 + t8204 + t8208 + t8298 + t8216 + t8218 + t8221 - t8237 - t8302 - t8305 + t8307;
    (t8293, t8295, t8296, t8298, t8300, t8302, t8303, t8305, t8307, t8308)
}
