//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 914/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk914<F: Float>(t4959: F, t7647: F, t30148: F, t5606: F, t7585: F, t7842: F, t1181: F, t23745: F, t604: F, t7493: F, t31362: F, t8775: F, t23736: F, t7351: F, t7575: F, t30268: F, t8956: F) -> (F, F, F, F, F, F) {
    let t34091 = t7647 * t4959;
    let t34092 = 0.17149607247227894789e-2 * t34091;
    let t34095 = t7585 * t7842 * t30148 * t5606;
    let t34099 = t7493 * t1181 * t604 * t23745;
    let t34100 = 0.21437009059034868486e-2 * t34099;
    let t34101 = t31362 * t8775;
    let t34102 = 0.10718504529517434243e-2 * t34101;
    let t34105 = t7575 * t1181 * t7351 * t23736;
    let t34107 = t30268 * t8956;
    (t34092, t34095, t34100, t34102, t34105, t34107)
}
