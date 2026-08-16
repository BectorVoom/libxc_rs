//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 945/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk945(t11513: f64, t1743: f64, t1749: f64, t190: f64, t632: f64, t11449: f64, t11451: f64, t5117: f64, t1: f64, t8820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11514 = t1743 * t11513;
    let t11515 = t11514 * t1749;
    let t11517 = t632 * t190;
    let t11518 = t11517 * t11449;
    let t11519 = t11451 * t5117;
    let t11520 = t11518 * t11519;
    let t11522 = t8820 * t1;
    (t11514, t11515, t11517, t11518, t11519, t11520, t11522)
}
