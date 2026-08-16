//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 870/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk870(t1987: f64, t3626: f64, t2849: f64, t2865: f64, t730: f64, t1976: f64, t3604: f64, t2874: f64, t1954: f64, t723: f64, t2873: f64, t7299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9347 = 0.17315859105681463759e2_f64 * t1987 * t3626;
    let t9348 = t2865 * t2849;
    let t9350 = 0.23392894490538584828e1_f64 * t730 * t9348;
    let t9351 = t1976 * t3604;
    let t9352 = t9351 * t2874;
    let t9354 = 0.17315859105681463759e2_f64 * t730 * t9352;
    let t9355 = t1954 * t3604;
    let t9356 = t9355 * t723;
    let t9358 = 0.11696447245269292414e1_f64 * t730 * t9356;
    let t9359 = t2873 * t7299;
    (t9347, t9348, t9350, t9351, t9352, t9354, t9356, t9358, t9359)
}
