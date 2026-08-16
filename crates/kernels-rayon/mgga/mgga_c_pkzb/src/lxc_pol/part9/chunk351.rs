//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 351/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk351(t1227: f64, t405: f64, t921: f64, t758: f64, t18: f64, t398: f64, t19: f64) -> (f64, f64, f64, f64) {
    let t1228 = t405 * t1227;
    let t1229 = t1228 * t921;
    let t1230 = t758 * t1229;
    let t1233 = t398 * t18;
    let t1235 = 1.0_f64 / t19 / t1233;
    (t1228, t1229, t1230, t1235)
}
