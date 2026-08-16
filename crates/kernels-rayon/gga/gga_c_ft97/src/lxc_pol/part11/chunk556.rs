//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 556/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk556(t1604: f64, t7839: f64, t1614: f64, t408: f64, t1608: f64, t373: f64, t1619: f64, t397: f64, t428: f64, t1618: f64, t388: f64, t401: f64, t409: f64) -> (f64, f64, f64, f64, f64) {
    let t7840 = t1604 * t7839;
    let t7843 = t408 * t1614;
    let t7845 = t1608 * t7843 * t373;
    let t7847 = t1619 * t397 * t428;
    let t7848 = t1618 * t7847;
    let t7852 = t388 * t409 * t401;
    (t7840, t7843, t7845, t7848, t7852)
}
