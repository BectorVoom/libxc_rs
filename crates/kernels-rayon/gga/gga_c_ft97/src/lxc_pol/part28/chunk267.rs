//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 267/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk267(t630: f64, t70: f64, t41: f64, t639: f64, t71: f64, t178: f64, t1638: f64, t1537: f64, t947: f64, t1546: f64, t89: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2264 = t630 * t70;
    let t2265 = t41 * t2264;
    let t2266 = t71 * t639;
    let t2280 = t178 * t178;
    let t2281 = 1.0_f64 / t2280;
    let t2289 = 0.19257444444444444444e0_f64 * t1638;
    let t2976 = t1537 * t947;
    let t2981 = t89 * t1546 * t921;
    (t2265, t2266, t2281, t2289, t2976, t2981)
}
