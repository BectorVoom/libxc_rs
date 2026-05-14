//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 915/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk915<F: Float>(t34569: F, t8465: F, t1992: F, t30692: F, t5720: F, t7842: F, t30364: F, t5147: F, t8901: F, t30689: F, t4967: F, t525: F, t864: F, t1165: F, t31567: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t35997 = t34569 * t8465;
    let t35998 = 0.94344276868812456204e-2 * t35997;
    let t36004 = t30692 * t7842 * t1992 * t5720;
    let t36005 = 0.10482697429868050689e-2 * t36004;
    let t36006 = t30364 * t5147;
    let t36007 = 0.17149607247227894789e-2 * t36006;
    let t36010 = t30692 * t7842 * t1992 * t8901;
    let t36011 = 0.10482697429868050689e-2 * t36010;
    let t36017 = t30689 * t4967;
    let t36018 = 0.34299214494455789578e-2 * t36017;
    let t36019 = t525 * t864;
    let t36022 = t31567 * t1165 * t604 * t36019;
    (t35998, t36005, t36007, t36011, t36018, t36019, t36022)
}
