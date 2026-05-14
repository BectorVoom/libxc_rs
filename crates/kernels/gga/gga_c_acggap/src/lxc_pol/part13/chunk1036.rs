//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1036/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1036<F: Float>(t7839: F, t8970: F, t1165: F, t2068: F, t33706: F, t604: F, t7337: F, t7338: F, t8480: F, t1181: F, t20595: F, t599: F, t20590: F, t31567: F, t36019: F, t1992: F, t7585: F, t7586: F, t8960: F) -> (F, F, F, F, F, F, F) {
    let t36096 = t7839 * t8970;
    let t36097 = 0.31448092289604152068e-3 * t36096;
    let t36100 = t2068 * t1165 * t604 * t33706;
    let t36103 = t7337 * t8480 * t7338;
    let t36107 = t7337 * t1181 * t599 * t20595;
    let t36111 = t7337 * t1181 * t599 * t20590;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    (t36097, t36100, t36103, t36107, t36111, t36115, t36119)
}
