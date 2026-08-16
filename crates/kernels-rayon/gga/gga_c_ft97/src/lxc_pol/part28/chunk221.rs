//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 221/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk221(t1614: f64, t77: f64, t373: f64, t1608: f64, t51: f64, t53: f64, t397: f64, t371: f64, t409: f64, t29: f64, t30: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1615 = t77 * t1614;
    let t1616 = t1615 * t373;
    let t1617 = t1608 * t1616;
    let t1619 = t51 * t53;
    let t1620 = t1619 * t397;
    let t1624 = t371 * t409;
    let t1630 = 1.0_f64 / t30 / t29;
    let t1631 = t25 * t1630;
    (t1616, t1617, t1620, t1624, t1630, t1631)
}
