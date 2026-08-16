//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1125/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1125(t127680: f64, t35917: f64, t35504: f64, t52: f64, t811: f64, t150590: f64, t31465: f64, t140943: f64, t33941: f64, t35932: f64, t33934: f64, t35928: f64) -> (f64, f64, f64, f64, f64) {
    let t153188 = t35917 * t127680;
    let t153193 = t52 * t35504 * t811;
    let t153196 = t31465 * t150590;
    let t153205 = t33941 * t140943 * t35932;
    let t153208 = t33934 * t140943 * t35928;
    (t153188, t153193, t153196, t153205, t153208)
}
