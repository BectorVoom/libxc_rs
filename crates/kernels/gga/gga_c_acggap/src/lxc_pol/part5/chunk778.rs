//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 778/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk778<F: Float>(t1679: F, t467: F, t6614: F, t1713: F, t192: F, t301: F, t96: F, t695: F, t1674: F, t1662: F, t1680: F, t130: F, t595: F, t594: F, t8: F, t1024: F, t56: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6616 = t1679 * t6614 * t467;
    let t6619 = t96 * t301 * t192 * t1713;
    let t6621 = t695 * t1713;
    let t6622 = t1674 * t6621;
    let t6625 = t1679 * t1680 * t1662;
    let t7309 = t130 * t595;
    let t7321 = 1.0 / t8 / t594;
    let t7322 = t130 * t7321;
    let t7335 = t56 * t1024;
    (t6616, t6619, t6621, t6622, t6625, t7309, t7321, t7322, t7335)
}
