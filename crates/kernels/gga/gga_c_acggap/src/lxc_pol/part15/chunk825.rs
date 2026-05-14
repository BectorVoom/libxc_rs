//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 825/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk825<F: Float>(t151: F, t37: F, t56: F, t593: F, t7508: F, t141: F, t420: F, t1: F, t203: F, t3157: F, t174: F, t172: F, t435: F, t7746: F, t993: F, t130: F, t1964: F) -> (F, F, F, F, F) {
    let t31009 = t151 * t593 / t7508 / t37 * t56;
    let t31010 = t420 * t141;
    let t31013 = t3157 * t1 * t203;
    let t31015 = t31009 * t31010 * t174 * t31013;
    let t31020 = t31009 * t420 * t172 * t435 * t31013;
    let t31022 = t7746 * t993;
    let t31035 = t130 * t1964;
    (t31010, t31015, t31020, t31022, t31035)
}
