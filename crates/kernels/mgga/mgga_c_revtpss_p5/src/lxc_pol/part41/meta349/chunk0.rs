//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1157/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1157<F: Float>(t1647: F, t3298: F, t1086: F, t1678: F, t994: F, t12166: F, t378: F, t342: F, t11631: F, t12050: F, t12077: F, t3154: F) -> (F, F, F, F, F, F) {
    let t16509 = t1647 * t3298;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16551 = t12166 * t378;
    let t16552 = t342 * t16551;
    let t16553 = t12050 * t11631;
    let t16558 = t12077 * t378;
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    (t16509, t16544, t16552, t16553, t16559, t16560)
}
