//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 329/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk329(t1416: f64, t152: f64, t172: f64, t19: f64, t20: f64, t435: f64, t505: f64) -> (f64, f64, f64, f64, f64) {
    let t1417 = t1416 * t152;
    let t1418 = t172 * t19;
    let t1419 = t1418 * t20;
    let t1420 = t1417 * t1419;
    let t1423 = t435 * t505;
    (t1417, t1418, t1419, t1420, t1423)
}
