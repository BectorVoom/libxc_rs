//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1065/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1065<F: Float>(t1873: F, t26004: F, t1513: F, t25823: F, t1497: F, t1927: F, t1470: F, t2247: F, t197: F, t530: F, t2013: F, t8995: F) -> (F, F, F, F, F, F) {
    let t27955 = t26004 * t1873;
    let t28034 = t25823 * t1513;
    let t28150 = t1927 * t1497;
    let t28154 = t2247 * t1470;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    (t27955, t28034, t28150, t28154, t28167, t28196)
}
