//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 743/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk743(t11392: f64, t24: f64, t469: f64, t3155: f64, t458: f64, t1771: f64, t963: f64, t358: f64, t378: f64, t93: f64, t1587: f64, t1755: f64, t3149: f64) -> (f64, f64, f64, f64, f64) {
    let t11665 = t24 * t469 * t11392;
    let t11668 = 2.0_f64 / 3.0_f64 * t458 * t3155;
    let t11669 = t1771 * t963;
    let t11672 = t378 * t93 * t358;
    let t11676 = t1587 * t3149 * t1755;
    (t11665, t11668, t11669, t11672, t11676)
}
