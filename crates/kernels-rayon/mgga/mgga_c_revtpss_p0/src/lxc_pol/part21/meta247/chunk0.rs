//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1427/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1427(t730: f64, t9446: f64, t2596: f64, t675: f64, t215: f64, t723: f64, t2553: f64, t738: f64, t2491: f64, t177: f64, t9417: f64, t2495: f64, t9368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9447 = t9446 * t730;
    let t9450 = t675 * t2596;
    let t9454 = t215 * t723;
    let t9461 = t675 * t2553;
    let t9469 = t215 * t738;
    let t9476 = t675 * t2491;
    let t9480 = t177 * t9417;
    let t9481 = t9368 * t2495;
    (t9447, t9450, t9454, t9461, t9469, t9476, t9480, t9481)
}
