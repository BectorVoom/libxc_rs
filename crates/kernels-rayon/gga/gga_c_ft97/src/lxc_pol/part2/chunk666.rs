//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 666/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk666(t9698: f64, t1636: f64, t714: f64, t89: f64, t191: f64, t7514: f64, t2336: f64, t2366: f64, t2344: f64, t375: f64, t2350: f64, t2374: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9699 = 14.0_f64 / 81.0_f64 * t9698;
    let t9701 = t89 * t1636 * t714;
    let t9707 = t191 * t7514;
    let t9723 = t89 * t2336 * t2366;
    let t9725 = t375 * t2344;
    let t9727 = t89 * t9725 * t2350;
    let t9730 = t89 * t375 * t2374;
    (t9699, t9701, t9707, t9723, t9725, t9727, t9730)
}
