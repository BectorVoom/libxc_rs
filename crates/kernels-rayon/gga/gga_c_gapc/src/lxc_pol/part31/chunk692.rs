//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 692/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk692(t125: f64, t2207: f64, t291: f64, t667: f64, t2232: f64, t442: f64, t1474: f64, t268: f64, t122: f64, t2435: f64, t1971: f64, t786: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6146 = t2207 * t125;
    let t6148 = t667 * t291;
    let t6172 = t2232 * t442;
    let t6178 = t1474 * t268;
    let t6179 = t2435 * t122;
    let t6181 = t1971 * t291;
    let t6182 = t830 * t786;
    (t6146, t6148, t6172, t6178, t6179, t6181, t6182)
}
