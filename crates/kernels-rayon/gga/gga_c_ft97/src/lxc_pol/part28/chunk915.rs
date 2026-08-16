//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 915/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk915(t25: f64, t408: f64, t22825: f64, t5517: f64, t1614: f64, t391: f64, t1602: f64, t1711: f64, t409: f64, t5532: f64, t388: f64, t7888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92377 = t408 * t25;
    let t92399 = t5517 * t22825;
    let t92461 = t1614 * t391;
    let t92470 = t1602 * t1711;
    let t92488 = t409 * t5532;
    let t92489 = t388 * t92488;
    let t92596 = t388 * t7888;
    (t92377, t92399, t92461, t92470, t92489, t92596)
}
