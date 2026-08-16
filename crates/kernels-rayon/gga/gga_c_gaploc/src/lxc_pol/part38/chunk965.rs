//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 965/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk965(t41884: f64, t11549: f64, t20535: f64, t2478: f64, t38019: f64, t544: f64, t9287: f64, t1429: f64, t2365: f64, t35888: f64, t35893: f64, t4391: f64) -> (f64, f64, f64, f64, f64) {
    let t46327 = 0.71500979903700853339e0_f64 * t41884;
    let t46331 = t20535 * t11549 * t2478;
    let t46335 = t544 * t38019 * t9287;
    let t46336 = 0.14896037479937677779e-1_f64 * t46335;
    let t46338 = t1429 * t2365 * t35888;
    let t46339 = 0.44688112439813033337e-1_f64 * t46338;
    let t46341 = t4391 * t2365 * t35893;
    (t46327, t46331, t46336, t46339, t46341)
}
