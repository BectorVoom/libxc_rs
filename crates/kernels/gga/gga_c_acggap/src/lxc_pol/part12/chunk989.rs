//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 989/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk989<F: Float>(t1181: F, t20595: F, t599: F, t7337: F, t20590: F, t31567: F, t36019: F, t1992: F, t7585: F, t7586: F, t8960: F, t30148: F, t7842: F, t8906: F, t1451: F, t7614: F) -> (F, F, F, F, F, F) {
    let t36107 = t7337 * t1181 * t599 * t20595;
    let t36111 = t7337 * t1181 * t599 * t20590;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    let t36123 = t7585 * t7842 * t30148 * t8906;
    let t36125 = t7614 * t1451;
    (t36107, t36111, t36115, t36119, t36123, t36125)
}
