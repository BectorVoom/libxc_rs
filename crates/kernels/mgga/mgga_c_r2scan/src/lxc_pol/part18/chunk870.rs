//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 870/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk870<F: Float>(t11056: F, t819: F, t3370: F, t833: F, t1074: F, t1299: F, t1338: F, t3416: F, t1096: F, t6755: F, t1348: F, t6767: F, t2867: F, t481: F, t3263: F, t3262: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11057 = t819 * t11056;
    let t11058 = 11.0 / 9.0 * t11057;
    let t11063 = t3370 * t833;
    let t11066 = t1074 * t1299;
    let t11145 = t1338 * t3416;
    let t11148 = t6755 * t1096;
    let t11157 = t1348 * t3416;
    let t11162 = t6767 * t1096;
    let t11475 = t2867 * t481;
    let t11476 = t3263 * t11475;
    let t11477 = t3262 * t11476;
    (t11058, t11063, t11066, t11145, t11148, t11157, t11162, t11475, t11476, t11477)
}
