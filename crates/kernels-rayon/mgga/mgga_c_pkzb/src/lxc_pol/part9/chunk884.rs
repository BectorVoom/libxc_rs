//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 884/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk884(t2099: f64, t2397: f64, t2395: f64, t6112: f64, t6119: f64, t6126: f64, t6134: f64, t6136: f64, t6139: f64, t6146: f64, t6228: f64, t6236: f64, t6243: f64, t6245: f64) -> (f64, f64, f64) {
    let t6491 = t2099 * t2397;
    let t6492 = t2395 * t6491;
    let t6494 = -t6112 - t6236 - t6228 + t6126 - t6243 - t6245 - t6119 + t6134 + t6136 + t6139 - t6146;
    (t6491, t6492, t6494)
}
