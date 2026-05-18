//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 632/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk632<F: Float>(t1569: F, t997: F, t1101: F, t1165: F, t540: F, t3361: F, t535: F, t1181: F, t1096: F, t4643: F, t1111: F, t3391: F) -> (F, F, F, F, F, F, F, F) {
    let t4901 = t997 * t1569;
    let t4904 = t1165 * t540 * t1101;
    let t4906 = F::new(0.34299214494455789578e-2) * t3361 * t4904;
    let t4907 = t535 * t1101;
    let t4908 = t1181 * t4907;
    let t4910 = F::new(0.34299214494455789578e-2) * t3361 * t4908;
    let t4911 = t4643 * t1096;
    let t4912 = t1181 * t4911;
    let t4915 = t4643 * t1111;
    let t4916 = t1181 * t4915;
    let t4918 = F::new(0.17149607247227894789e-2) * t3391 * t4916;
    (t4901, t4904, t4906, t4908, t4910, t4912, t4916, t4918)
}
