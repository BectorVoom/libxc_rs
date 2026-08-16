//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 908/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk908(t11362: f64, t9333: f64, t1457: f64, t1572: f64, t44480: f64, t44470: f64, t13421: f64, t1641: f64, t13363: f64, t37777: f64, t11359: f64, t40342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46291 = 0.10725146985555128001e1_f64 * t11362 * t9333;
    let t46294 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t44480;
    let t46297 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t44470;
    let t46299 = 0.92023022289409799224e1_f64 * t1641 * t13421;
    let t46301 = 0.42900587942220512003e1_f64 * t37777 * t13363;
    let t46303 = 0.42900587942220512003e1_f64 * t11359 * t40342;
    (t46291, t46294, t46297, t46299, t46301, t46303)
}
