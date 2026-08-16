//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1200/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1200(t1160: f64, t17927: f64, t284: f64, t55330: f64, t9102: f64, t9104: f64, t1179: f64, t54615: f64, t1162: f64, t17666: f64, t2367: f64, t9116: f64, t9118: f64) -> (f64, f64, f64, f64, f64) {
    let t55337 = t1160 * t17927 * t284;
    let t55341 = t9102 * t55330 * t9104;
    let t55343 = t1179 * t54615;
    let t55346 = t1162 * t2367 * t17666;
    let t55361 = t9116 * t55330 * t9118;
    (t55337, t55341, t55343, t55346, t55361)
}
