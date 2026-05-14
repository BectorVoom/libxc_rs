//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1117/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1117<F: Float>(t19318: F, t2924: F, t1610: F, t4631: F, t2874: F, t6145: F, t934: F, t11299: F, t6142: F, t2926: F, t6141: F, t11466: F, t11507: F, t19294: F, t19297: F, t19300: F, t19304: F, t19307: F, t19311: F, t19315: F, t19317: F, t2987: F, t3012: F) -> (F, F, F, F, F, F) {
    let t19320 = 6.0 * t2924 * t19318;
    let t19321 = t1610 * t4631;
    let t19323 = 4.0 * t2874 * t19321;
    let t19324 = t6145 * t934;
    let t19326 = 0.96491876992155210402e2 * t11299 * t19324;
    let t19327 = t6142 * t934;
    let t19329 = 2.0 * t2874 * t19327;
    let t19330 = t6141 * t2926;
    let t19331 = t19330 * t934;
    let t19333 = 0.16081979498692535067e2 * t2924 * t19331;
    let t19334 = -0.23392894490538584828e1 * t2987 * t19294 - 0.10389515463408878255e3 * t11466 * t19297 - 0.11696447245269292414e1 * t2987 * t19300 + 0.17315859105681463759e2 * t3012 * t19304 + 0.34631718211362927518e2 * t3012 * t19307 + 0.10254018858216406658e4 * t11507 * t19311 + t19315 - t19317 - t19320 + t19323 + t19326 + t19329 - t19333;
    (t19320, t19323, t19326, t19329, t19333, t19334)
}
