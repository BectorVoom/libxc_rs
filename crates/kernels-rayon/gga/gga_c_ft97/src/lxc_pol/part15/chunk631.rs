//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 631/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk631(t1546: f64, t4426: f64, t89: f64, t4432: f64, t1597: f64, t4441: f64, t534: f64, t408: f64, t4491: f64, t1710: f64, t4474: f64, t8051: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    let t15630 = t4441 * t1597;
    let t15680 = t534 * t15630;
    let t15706 = t408 * t4491;
    let t15712 = t1710 * t4474;
    let t15716 = t8051 * t4474;
    (t15609, t15612, t15630, t15680, t15706, t15712, t15716)
}
