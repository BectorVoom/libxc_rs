//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 545/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk545<F: Float>(t3361: F, t4904: F, t1101: F, t535: F, t1181: F, t1111: F, t4643: F, t3391: F, t1165: F, t1532: F, t4183: F, t3451: F, t1541: F, t3372: F, t1005: F, t1352: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4906 = 0.34299214494455789578e-2 * t3361 * t4904;
    let t4907 = t535 * t1101;
    let t4908 = t1181 * t4907;
    let t4910 = 0.34299214494455789578e-2 * t3361 * t4908;
    let t4915 = t4643 * t1111;
    let t4916 = t1181 * t4915;
    let t4918 = 0.17149607247227894789e-2 * t3391 * t4916;
    let t4925 = t1165 * t1532 * t4183;
    let t4926 = t3451 * t4925;
    let t4928 = t3372 * t1541;
    let t4946 = t1005 * t1352;
    (t4906, t4908, t4910, t4916, t4918, t4925, t4926, t4928, t4946)
}
