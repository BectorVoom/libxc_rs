//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1169/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1169<F: Float>(t1992: F, t30692: F, t5720: F, t7842: F, t30364: F, t5147: F, t8901: F, t1181: F, t2068: F, t23045: F, t604: F, t30689: F, t4967: F) -> (F, F, F, F, F) {
    let t36004 = t30692 * t7842 * t1992 * t5720;
    let t36005 = F::cast_from(0.10482697429868050689e-2_f64) * t36004;
    let t36006 = t30364 * t5147;
    let t36007 = F::cast_from(0.17149607247227894789e-2_f64) * t36006;
    let t36010 = t30692 * t7842 * t1992 * t8901;
    let t36011 = F::cast_from(0.10482697429868050689e-2_f64) * t36010;
    let t36014 = t2068 * t1181 * t604 * t23045;
    let t36017 = t30689 * t4967;
    (t36005, t36007, t36011, t36014, t36017)
}
