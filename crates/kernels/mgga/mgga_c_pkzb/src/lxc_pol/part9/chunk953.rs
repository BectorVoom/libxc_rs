//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 953/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk953<F: Float>(t3160: F, t6337: F, t898: F, t2313: F, t3152: F, t8098: F, t881: F, t890: F, t2298: F, t2328: F, t3153: F, t8147: F, t8185: F, t8187: F, t8191: F, t8194: F, t8197: F, t8201: F, t8204: F, t8208: F, t8216: F, t8218: F, t8221: F, t8237: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8293 = t3160 * t6337;
    let t8295 = 0.17315859105681463759e2 * t898 * t8293;
    let t8296 = t3152 * t2313;
    let t8298 = 0.11696447245269292414e1 * t898 * t8296;
    let t8300 = t881 * t8098 * t890;
    let t8302 = 0.5848223622634646207e0 * t898 * t8300;
    let t8303 = t3160 * t2298;
    let t8305 = 0.35089341735807877242e1 * t898 * t8303;
    let t8307 = 0.23392894490538584828e1 * t2328 * t3153;
    let t8308 = -t8295 + t8147 - t8185 + t8187 - t8191 - t8194 - t8197 + t8201 + t8204 + t8208 + t8298 + t8216 + t8218 + t8221 - t8237 - t8302 - t8305 + t8307;
    (t8293, t8295, t8296, t8298, t8300, t8302, t8303, t8305, t8307, t8308)
}
