//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 905/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk905<F: Float>(t13633: F, t151: F, t395: F, t409: F, t1103: F, t3700: F, t3570: F, t962: F, t1077: F, t336: F, t1163: F, t1181: F, t3169: F, t991: F) -> (F, F, F, F, F, F) {
    let t13635 = t151 * t395 * t13633;
    let t13636 = t13635 * t409;
    let t13638 = t3700 * t1103;
    let t13654 = t3570 * t962;
    let t13656 = t336 * t1077;
    let t13664 = t1163 * t1181 * t991 * t3169;
    (t13635, t13636, t13638, t13654, t13656, t13664)
}
