//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1036/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1036(t22366: f64, t22422: f64, t22485: f64, t22617: f64, t40: f64, t60: f64, t544: f64, t6525: f64, t1872: f64, t2045: f64, t22026: f64, t88: f64) -> (f64, f64, f64, f64) {
    let t22621 = t40 * t60 * (t22366 + t22422 + t22485 + t22617);
    let t22623 = 16.0_f64 * t544 * t6525;
    let t22624 = t2045 * t1872;
    let t22625 = 72.0_f64 * t22624;
    let t22626 = t22026 * t88;
    (t22621, t22623, t22625, t22626)
}
