//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 954/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk954<F: Float>(t11797: F, t1227: F, t248: F, t3248: F, t3521: F, t1009: F, t3481: F, t1011: F, t1212: F, t486: F, t676: F, t1216: F) -> (F, F, F, F, F, F) {
    let t11798 = t1227 * t11797;
    let t11801 = t248 * t3521 * t3248;
    let t11802 = t1227 * t11801;
    let t11812 = t3481 * t1009;
    let t11813 = t11812 * t1011;
    let t11814 = t11813 * t1212;
    let t11818 = t676 * t486;
    let t11820 = t248 * t11818 * t1216;
    (t11798, t11802, t11812, t11814, t11818, t11820)
}
