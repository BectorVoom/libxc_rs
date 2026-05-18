//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 870/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk870<F: Float>(t1987: F, t3626: F, t2849: F, t2865: F, t730: F, t1976: F, t3604: F, t2874: F, t1954: F, t723: F, t2873: F, t7299: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9347 = F::new(0.17315859105681463759e2) * t1987 * t3626;
    let t9348 = t2865 * t2849;
    let t9350 = F::new(0.23392894490538584828e1) * t730 * t9348;
    let t9351 = t1976 * t3604;
    let t9352 = t9351 * t2874;
    let t9354 = F::new(0.17315859105681463759e2) * t730 * t9352;
    let t9355 = t1954 * t3604;
    let t9356 = t9355 * t723;
    let t9358 = F::new(0.11696447245269292414e1) * t730 * t9356;
    let t9359 = t2873 * t7299;
    (t9347, t9348, t9350, t9351, t9352, t9354, t9356, t9358, t9359)
}
