//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 711/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk711<F: Float>(t3300: F, t398: F, t5094: F, t513: F, t864: F, t1095: F, t1036: F, t1032: F, t1434: F, t506: F, t922: F, t1426: F, t368: F) -> (F, F, F, F, F, F, F) {
    let t5096 = t398 * t3300 * t5094;
    let t5099 = t513 * t864;
    let t5101 = t398 * t1095 * t5099;
    let t5102 = t1036 * t5101;
    let t5104 = t1032 * t1434;
    let t5106 = t506 * t922;
    let t5108 = t1426 * t368 * t5106;
    (t5096, t5099, t5101, t5102, t5104, t5106, t5108)
}
