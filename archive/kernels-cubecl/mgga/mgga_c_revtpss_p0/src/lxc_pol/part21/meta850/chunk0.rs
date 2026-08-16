//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3192/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3192<F: Float>(t1261: F, t1715: F, t247: F, t44701: F, t1214: F, t17748: F, t17754: F, t12809: F, t12916: F, t17380: F, t3568: F, t5333: F) -> (F, F, F, F, F) {
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58780 = t17748 * t1214;
    let t58785 = t17754 * t1214;
    let t58791 = t12809 * t12916 * t17380;
    let t58793 = t5333 * t3568;
    (t58777, t58780, t58785, t58791, t58793)
}
