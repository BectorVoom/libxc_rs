//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 855/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk855<F: Float>(t11995: F, t12058: F, t12091: F, t12144: F, t40: F, t60: F, t11870: F, t272: F, t2773: F, t286: F, t699: F, t712: F) -> (F, F, F) {
    let t12148 = t40 * t60 * (t11995 + t12058 + t12091 + t12144);
    let t12156 = F::cast_from(0.14035736694323150897e2_f64) * t286 * t2773 * t11870 * t272;
    let t12157 = t712 * t699;
    (t12148, t12156, t12157)
}
