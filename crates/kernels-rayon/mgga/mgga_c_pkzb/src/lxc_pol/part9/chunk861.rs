//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 861/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk861(t6122: f64, t6230: f64, t6233: f64, t898: f64, t6112: f64, t6116: f64, t6119: f64, t6126: f64, t6134: f64, t6136: f64, t6139: f64, t6146: f64, t6196: f64, t6204: f64, t6207: f64, t6228: f64) -> (f64, f64, f64) {
    let t6234 = t6230 * t6122 * t6233;
    let t6236 = 0.10254018858216406658e4_f64 * t898 * t6234;
    let t6237 = -t6112 + t6116 - t6119 + t6126 + t6134 + t6136 + t6139 - t6146 + t6196 + t6204 + t6207 - t6228 - t6236;
    (t6234, t6236, t6237)
}
