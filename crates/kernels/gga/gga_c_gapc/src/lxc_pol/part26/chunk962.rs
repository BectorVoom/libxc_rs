//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 962/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk962<F: Float>(t11875: F, t1453: F, t7949: F, t818: F, t959: F, t1736: F, t640: F, t7073: F, t1086: F, t2211: F, t9388: F, t1: F, t128: F, t2580: F, t350: F, t126: F, t15541: F, t190: F, t1903: F, t314: F, t442: F, t7953: F) -> (F, F, F, F, F) {
    let t33595 = t11875 * t1453 * t818 * t959 * t7949;
    let t33597 = t640 * t1736;
    let t33598 = t7073 * t33597;
    let t33601 = t33598 * t1086 * t2211 * t9388;
    let t33606 = t33598 * t2580 * t128 * t1 * t350;
    let t33614 = t7953 * t126 * t1903 * t15541 * t314 * t190 * t442;
    (t33595, t33597, t33601, t33606, t33614)
}
