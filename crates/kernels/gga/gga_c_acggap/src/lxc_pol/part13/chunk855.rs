//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 855/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk855<F: Float>(t1095: F, t31539: F, t7457: F, t7458: F, t2104: F, t7780: F, t2067: F, t3073: F, t1165: F, t15407: F, t604: F, t3088: F, t15758: F, t1181: F, t16020: F, t599: F, t7346: F) -> (F, F, F, F, F, F, F) {
    let t31542 = t7457 * t7458 * t1095 * t31539;
    let t31543 = 0.31448092289604152067e-3 * t31542;
    let t31544 = t7780 * t2104;
    let t31562 = t3073 * t2067;
    let t31565 = t31562 * t1165 * t604 * t15407;
    let t31567 = t3088 * t2067;
    let t31570 = t31567 * t1165 * t604 * t15758;
    let t31585 = t7346 * t1181 * t599 * t16020;
    (t31543, t31544, t31562, t31565, t31567, t31570, t31585)
}
