//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1073/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1073<F: Float>(t30546: F, t8606: F, t1165: F, t30327: F, t4358: F, t604: F, t30861: F, t8458: F, t5265: F, t7351: F, t8463: F, t1181: F, t5106: F) -> (F, F, F, F, F) {
    let t34718 = t30546 * t8606;
    let t34722 = t30327 * t1165 * t604 * t4358;
    let t34724 = t30861 * t8458;
    let t34728 = t8463 * t1165 * t7351 * t5265;
    let t34732 = t8463 * t1181 * t604 * t5106;
    (t34718, t34722, t34724, t34728, t34732)
}
