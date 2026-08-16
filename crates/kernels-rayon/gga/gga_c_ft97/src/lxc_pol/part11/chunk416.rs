//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 416/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk416(t1557: f64, t179: f64, t1559: f64, t2258: f64, t630: f64, t70: f64, t41: f64) -> (f64, f64, f64, f64) {
    let t2259 = t179 * t1557;
    let t2261 = t2258 * t2259 * t1559;
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    (t2259, t2261, t2264, t2265)
}
