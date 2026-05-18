//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1212/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1212<F: Float>(t2860: F, t9359: F, t10979: F, t2029: F, t10937: F, t2887: F, t68: F, t10985: F, t2099: F, t5954: F, t10942: F, t17938: F) -> (F, F, F, F, F) {
    let t29753 = F::new(0.10389515463408878255e3) * t2860 * t9359;
    let t29754 = t10979 * t2029;
    let t29762 = t2887 * t68 * t10937;
    let t29766 = t5954 * t2099 * t10985;
    let t29775 = t10942 * t17938;
    (t29753, t29754, t29762, t29766, t29775)
}
