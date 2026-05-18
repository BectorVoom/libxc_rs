//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1030/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1030<F: Float>(t31362: F, t8903: F, t7839: F, t8908: F, t8912: F, t8970: F, t1181: F, t31567: F, t36019: F, t599: F, t1992: F, t7585: F, t7586: F, t8960: F) -> (F, F, F, F, F, F) {
    let t36085 = t31362 * t8903;
    let t36087 = t7839 * t8908;
    let t36089 = t7839 * t8912;
    let t36096 = t7839 * t8970;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    (t36085, t36087, t36089, t36096, t36115, t36119)
}
