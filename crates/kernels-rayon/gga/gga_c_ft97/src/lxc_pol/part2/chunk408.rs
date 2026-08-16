//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 408/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk408(t2253: f64, t634: f64, t645: f64, t422: f64, t70: f64, t1557: f64, t179: f64, t1559: f64, t630: f64, t41: f64) -> (f64, f64, f64, f64, f64) {
    let t2254 = t2253 * t634;
    let t2256 = t2253 * t645;
    let t2258 = t70 * t422;
    let t2259 = t179 * t1557;
    let t2261 = t2258 * t2259 * t1559;
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    (t2254, t2256, t2258, t2261, t2265)
}
