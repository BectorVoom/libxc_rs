//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 540/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk540(t1526: f64, t1529: f64, t7705: f64, t1533: f64, t342: f64, t630: f64, t1557: f64, t81: f64, t1559: f64, t1570: f64, t1528: f64, t1580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    let t7712 = t81 * t1557;
    let t7713 = t7712 * t1559;
    let t7720 = t81 * t1570;
    let t7721 = t7720 * t1559;
    let t7725 = t1528 * t1580;
    (t7707, t7710, t7712, t7713, t7721, t7725)
}
