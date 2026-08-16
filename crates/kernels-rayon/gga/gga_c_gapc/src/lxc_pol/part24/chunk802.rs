//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 802/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk802(t3284: f64, t7241: f64, t1092: f64, t2555: f64, t191: f64, t2786: f64, t3304: f64, t3278: f64, t3285: f64, t3289: f64, t3288: f64, t7178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9586 = t3284 * t7241;
    let t9587 = t1092 * t9586;
    let t9589 = t1092 * t2555;
    let t9591 = t2786 * t191;
    let t9592 = t9591 * t3304;
    let t9595 = t3278 * t3285;
    let t9597 = t3278 * t3289;
    let t9599 = t3288 * t7178;
    (t9586, t9587, t9589, t9592, t9595, t9597, t9599)
}
