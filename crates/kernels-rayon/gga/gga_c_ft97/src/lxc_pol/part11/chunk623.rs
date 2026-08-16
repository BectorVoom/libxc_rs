//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 623/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk623(t2282: f64, t643: f64, t637: f64, t8618: f64, t2281: f64, t2294: f64, t632: f64, t72: f64, t7745: f64, t1736: f64, t70: f64, t179: f64, t7763: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8619 = t2282 * t643;
    let t8621 = t637 * t8618 * t8619;
    let t8624 = t2281 * t643;
    let t8626 = t637 * t8624 * t2294;
    let t8630 = t72 * t632 * t7745;
    let t8633 = t70 * t1736;
    let t8634 = t179 * t7763;
    (t8619, t8621, t8624, t8626, t8630, t8633, t8634)
}
