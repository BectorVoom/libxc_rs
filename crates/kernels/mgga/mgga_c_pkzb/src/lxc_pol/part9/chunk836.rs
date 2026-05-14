//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 836/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk836<F: Float>(t385: F, t6377: F, t465: F, t931: F, t179: F, t824: F, t404: F, t2185: F, t2411: F, t758: F, t2405: F, t6106: F, t932: F, t53: F, t2226: F, t2410: F, t334: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6379 = 5.0 / 1296.0 * t385 * t6377;
    let t6380 = t465 * t931;
    let t6382 = t179 * t6380 * t824;
    let t6383 = t404 * t6382;
    let t6386 = t2411 * t824 * t2185;
    let t6387 = t758 * t6386;
    let t6391 = t179 * t2405 * t2185;
    let t6392 = t404 * t6391;
    let t6395 = t179 * t932 * t6106;
    let t6398 = t53 * t2411;
    let t6400 = t179 * t6398 * t2226;
    let t6401 = t404 * t6400;
    let t6404 = 1.0 / t2410 / t334;
    (t6379, t6380, t6382, t6383, t6386, t6387, t6391, t6392, t6395, t6400, t6401, t6404)
}
