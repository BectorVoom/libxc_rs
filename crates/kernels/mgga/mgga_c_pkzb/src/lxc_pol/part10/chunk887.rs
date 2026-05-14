//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 887/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk887<F: Float>(t179: F, t6380: F, t824: F, t404: F, t2185: F, t2405: F, t2411: F, t53: F, t2226: F, t2410: F, t334: F, t2370: F, t2029: F, t919: F, t2387: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6382 = t179 * t6380 * t824;
    let t6383 = t404 * t6382;
    let t6391 = t179 * t2405 * t2185;
    let t6392 = t404 * t6391;
    let t6398 = t53 * t2411;
    let t6400 = t179 * t6398 * t2226;
    let t6401 = t404 * t6400;
    let t6404 = 1.0 / t2410 / t334;
    let t6411 = t2370 * t824;
    let t6416 = t919 * t2029;
    let t6417 = t2370 * t2387;
    (t6382, t6383, t6391, t6392, t6398, t6400, t6401, t6404, t6411, t6416, t6417)
}
