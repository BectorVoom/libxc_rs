//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 607/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk607<F: Float>(t1095: F, t398: F, t5099: F, t1036: F, t1032: F, t1434: F, t506: F, t922: F, t1426: F, t368: F, t1487: F, t301: F, t1089: F, t372: F, t1083: F, t1539: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    let t5106 = t506 * t922;
    let t5108 = t1426 * t368 * t5106;
    let t5111 = t1487 * t301;
    let t5113 = t1089 * t368 * t5111;
    let t5116 = t1487 * t372;
    let t5118 = t398 * t1083 * t5116;
    let t5122 = t1539 * t360;
    (t5101, t5102, t5104, t5106, t5108, t5111, t5113, t5116, t5118, t5122)
}
