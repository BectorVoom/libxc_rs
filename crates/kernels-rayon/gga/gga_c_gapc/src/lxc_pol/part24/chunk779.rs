//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 779/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk779(t3008: f64, t9283: f64, t134: f64, t1403: f64, t3005: f64, t2998: f64, t3004: f64, t3007: f64, t9079: f64, t1404: f64, t2982: f64, t3084: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9284 = t9283 * t3008;
    let t9286 = t134 * t1403;
    let t9287 = t3005 * t9286;
    let t9288 = t2998 * t9287;
    let t9289 = t3004 * t9288;
    let t9291 = t9079 * t3007;
    let t9292 = t3004 * t9291;
    let t9294 = t2982 * t1404;
    let t9295 = t3084 * t9294;
    (t9284, t9288, t9289, t9291, t9292, t9294, t9295)
}
