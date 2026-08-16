//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 683/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk683(t56: f64, t6567: f64, t202: f64, t188: f64, t3649: f64, t3696: f64, t2211: f64, t723: f64, t2217: f64, t720: f64, t722: f64, t179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6568 = t6567 * t56;
    let t6569 = t6568 * t202;
    let t6571 = 455.0_f64 / 27.0_f64 * t188 * t6569;
    let t6576 = -0.60319259259259259259e1_f64 * t3649 - 0.54733333333333333333e-2_f64 * t3696;
    let t6578 = t2211 * t723;
    let t6581 = t720 * t2217;
    let t6586 = t722 * t722;
    let t6587 = 1.0_f64 / t6586;
    let t6588 = t179 * t6587;
    (t6568, t6569, t6571, t6576, t6578, t6581, t6586, t6587, t6588)
}
