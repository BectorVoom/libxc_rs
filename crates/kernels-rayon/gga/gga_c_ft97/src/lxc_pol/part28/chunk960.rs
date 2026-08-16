//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 960/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk960(t32047: f64, t342: f64, t630: f64, t1286: f64, t23047: f64, t32026: f64, t32029: f64, t1774: f64, t5700: f64, t7151: f64, t1526: f64, t5692: f64, t7705: f64) -> (f64, f64, f64, f64, f64) {
    let t137400 = t342 * t630 * t32047;
    let t137404 = t1286 * t23047;
    let t137412 = t32026 * t32029;
    let t137415 = t7151 * t1774 * t5700;
    let t137418 = t1526 * t7705 * t5692;
    (t137400, t137404, t137412, t137415, t137418)
}
