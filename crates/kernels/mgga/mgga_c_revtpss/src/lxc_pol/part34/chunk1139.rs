//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1139/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1139<F: Float>(t1513: F, t25823: F, t38: F, t7714: F, t2247: F, t13272: F, t6957: F, t1497: F, t1927: F, t1926: F, t1470: F, t197: F, t530: F) -> (F, F, F, F, F, F, F, F) {
    let t28034 = t25823 * t1513;
    let t28126 = t38 * t7714;
    let t28127 = t2247 * t28126;
    let t28138 = t13272 * t6957;
    let t28150 = t1927 * t1497;
    let t28151 = t1926 * t28150;
    let t28154 = t2247 * t1470;
    let t28166 = t197 * t530;
    (t28034, t28126, t28127, t28138, t28150, t28151, t28154, t28166)
}
