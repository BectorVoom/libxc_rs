//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 839/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk839<F: Float>(t31539: F, t368: F, t7457: F, t7458: F, t7310: F, t7386: F, t7637: F, t7753: F, t3077: F, t7646: F, t1167: F, t30861: F, t7495: F, t7676: F, t7720: F, t2092: F, t7630: F) -> (F, F, F, F, F, F, F, F) {
    let t31601 = t7457 * t7458 * t368 * t31539;
    let t31603 = t7310 * t7386;
    let t31605 = t7637 * t7753;
    let t31611 = t3077 * t7646;
    let t31612 = t31611 * t1167;
    let t31619 = t30861 * t7495;
    let t31625 = t7676 * t7720;
    let t31627 = t7630 * t2092;
    (t31601, t31603, t31605, t31611, t31612, t31619, t31625, t31627)
}
