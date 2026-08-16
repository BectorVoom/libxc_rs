//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 750/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk750(t5269: f64, t581: f64, t164: f64, t1719: f64, t179: f64, t568: f64, t1731: f64, t1773: f64, t1730: f64) -> (f64, f64, f64, f64) {
    let t5270 = t581 * t5269;
    let t5273 = t1719 * t164;
    let t5275 = t179 * t5273 * t568;
    let t5278 = t1731 * t1773;
    let t5279 = t1730 * t5278;
    (t5270, t5275, t5278, t5279)
}
