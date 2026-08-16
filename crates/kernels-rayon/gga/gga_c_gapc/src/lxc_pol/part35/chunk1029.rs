//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1029/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1029(t2404: f64, t3439: f64, t442: f64, t6172: f64, t2206: f64, t932: f64, t6851: f64, t761: f64, t147: f64, t19: f64, t2254: f64, t3296: f64) -> (f64, f64, f64, f64, f64) {
    let t24202 = t3439 * t442 * t2404;
    let t24271 = t3439 * t6172;
    let t24352 = t932 * t2206;
    let t24398 = t761 * t6851;
    let t24499 = t3296 * t2254 * t19 * t147;
    (t24202, t24271, t24352, t24398, t24499)
}
