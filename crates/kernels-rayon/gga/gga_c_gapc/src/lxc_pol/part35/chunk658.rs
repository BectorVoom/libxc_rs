//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 658/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk658(t203: f64, t618: f64, t1715: f64, t442: f64, t1457: f64, t169: f64) -> (f64, f64, f64) {
    let t5407 = t618 * t203;
    let t5408 = t1715 * t442;
    let t5409 = t5407 * t5408;
    let t5462 = t169 * t1457;
    (t5407, t5409, t5462)
}
