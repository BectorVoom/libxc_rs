//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 743/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk743(t10052: f64, t10053: f64, t242: f64, t2373: f64, t2574: f64, t773: f64, t1882: f64, t2576: f64, t2571: f64, t2619: f64, t713: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10054 = t10052 * t10053;
    let t10055 = t242 * t10054;
    let t10059 = t2574 * t773 * t2373;
    let t10062 = t1882 * t2576;
    let t10064 = t1882 * t2571;
    let t10067 = t729 * t2619 * t713;
    (t10054, t10055, t10059, t10062, t10064, t10067)
}
