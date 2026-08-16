//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1155/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1155<F: Float>(t1497: F, t1927: F, t1470: F, t2247: F, t197: F, t530: F, t2013: F, t531: F, t7933: F, t8995: F, t2033: F, t9593: F) -> (F, F, F, F, F, F) {
    let t28150 = t1927 * t1497;
    let t28154 = t2247 * t1470;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28172 = t531 * t7933;
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    (t28150, t28154, t28167, t28172, t28196, t28197)
}
