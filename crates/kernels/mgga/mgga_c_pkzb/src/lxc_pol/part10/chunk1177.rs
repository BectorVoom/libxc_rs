//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1177/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1177<F: Float>(t17928: F, t6513: F, t326: F, t17955: F, t918: F, t922: F, t179: F, t2185: F, t404: F, t6380: F, t2410: F, t344: F, t148: F, t931: F, t824: F, t2411: F, t465: F) -> (F, F, F, F, F, F, F, F) {
    let t19115 = t17928 * t6513;
    let t19116 = t19115 * t326;
    let t19124 = t918 * t17955 * t922;
    let t19128 = t404 * t179 * t6380 * t2185;
    let t19140 = 1.0 / t2410 / t344;
    let t19150 = t148 * t931;
    let t19153 = t404 * t179 * t19150 * t824;
    let t19155 = t465 * t2411;
    (t19115, t19116, t19124, t19128, t19140, t19150, t19153, t19155)
}
