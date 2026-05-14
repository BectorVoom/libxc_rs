//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 734/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk734<F: Float>(t40594: F, t44799: F, t9647: F, t16880: F, t35446: F, t11894: F, t2508: F, t7226: F, t7291: F, t2541: F, t36654: F, t11931: F, t2558: F, t943: F, t13552: F, t2549: F) -> (F, F, F, F, F, F) {
    let t44801 = t9647 * t44799 * t40594;
    let t44802 = 0.38452631150711121418e-2 * t44801;
    let t44804 = t9647 * t16880 * t35446;
    let t44805 = 0.19226315575355560709e-2 * t44804;
    let t44809 = 0.46143157380853345701e-1 * t2508 * t7226 * t11894 * t7291;
    let t44812 = 0.11535789345213336425e0 * t2508 * t2541 * t36654;
    let t44817 = t943 * t11931 * t2558;
    let t44818 = 0.32043859292259267849e-3 * t44817;
    let t44819 = t2549 * t13552;
    (t44802, t44805, t44809, t44812, t44818, t44819)
}
