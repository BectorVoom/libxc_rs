//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 665/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk665(t1185: f64, t851: f64, t2197: f64, t1174: f64, t2203: f64, t836: f64, t2175: f64, t2207: f64, t3017: f64, t3028: f64) -> (f64, f64, f64, f64, f64) {
    let t3038 = t1185 * t851;
    let t3040 = 2.0_f64 * t2197 * t3038;
    let t3041 = t2203 * t1174;
    let t3042 = t3041 * t836;
    let t3046 = t2207 - t2175 / 3.0_f64 - t3017 / 3.0_f64 + t3028;
    (t3038, t3040, t3041, t3042, t3046)
}
