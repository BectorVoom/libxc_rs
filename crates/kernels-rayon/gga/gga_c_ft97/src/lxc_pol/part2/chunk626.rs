//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 626/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk626(t1614: f64, t408: f64, t1608: f64, t373: f64, t1619: f64, t397: f64, t428: f64, t12: f64, t52: f64, t1703: f64, t1593: f64, t1609: f64) -> (f64, f64, f64, f64, f64) {
    let t7843 = t408 * t1614;
    let t7845 = t1608 * t7843 * t373;
    let t7847 = t1619 * t397 * t428;
    let t7853 = t52 * t12;
    let t7854 = t7853 * t1703;
    let t7857 = t1609 * t1593;
    (t7845, t7847, t7853, t7854, t7857)
}
