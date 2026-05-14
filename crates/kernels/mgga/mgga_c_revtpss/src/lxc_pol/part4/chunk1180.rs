//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1180/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1180<F: Float>(t16237: F, t380: F, t15780: F, t4998: F, t15893: F, t3304: F, t3318: F, t1086: F, t1678: F, t994: F, t12166: F, t378: F, t342: F, t11631: F, t12050: F, t3151: F) -> (F, F, F, F, F, F, F) {
    let t16529 = t380 * t16237;
    let t16534 = t15780 * t4998;
    let t16537 = t15893 * t3304;
    let t16540 = t15893 * t3318;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16551 = t12166 * t378;
    let t16552 = t342 * t16551;
    let t16553 = t12050 * t11631;
    let t16554 = t16553 * t3151;
    (t16529, t16534, t16537, t16540, t16544, t16552, t16554)
}
