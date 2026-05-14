//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 753/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk753<F: Float>(t11680: F, t2563: F, t9647: F, t123: F, t35439: F, t40594: F, t16880: F, t35446: F, t11894: F, t2508: F, t7226: F, t7291: F, t2541: F, t36654: F, t11931: F, t2558: F, t943: F) -> (F, F, F, F, F, F) {
    let t44797 = t9647 * t11680 * t2563;
    let t44798 = 0.22430701504581487494e-2 * t44797;
    let t44799 = t35439 * t123;
    let t44801 = t9647 * t44799 * t40594;
    let t44802 = 0.38452631150711121418e-2 * t44801;
    let t44804 = t9647 * t16880 * t35446;
    let t44805 = 0.19226315575355560709e-2 * t44804;
    let t44809 = 0.46143157380853345701e-1 * t2508 * t7226 * t11894 * t7291;
    let t44812 = 0.11535789345213336425e0 * t2508 * t2541 * t36654;
    let t44817 = t943 * t11931 * t2558;
    (t44798, t44802, t44805, t44809, t44812, t44817)
}
