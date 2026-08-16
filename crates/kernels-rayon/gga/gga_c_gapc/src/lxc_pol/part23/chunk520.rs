//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 520/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk520(t191: f64, t424: f64, t1046: f64, t1938: f64, t599: f64, t596: f64, t1936: f64, t611: f64, t1894: f64, t618: f64, t646: f64, t1026: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3028 = t424 * t191;
    let t3029 = t3028 * t1046;
    let t3031 = t1938 * t599;
    let t3032 = t596 * t3031;
    let t3034 = t611 * t1936;
    let t3035 = t618 * t1894;
    let t3036 = t646 * t3035;
    let t3037 = t3034 * t3036;
    let t3039 = t633 * t1026;
    (t3028, t3029, t3031, t3032, t3034, t3036, t3037, t3039)
}
