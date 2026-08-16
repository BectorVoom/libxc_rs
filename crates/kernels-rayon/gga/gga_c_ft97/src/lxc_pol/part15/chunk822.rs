//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 822/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk822(t19233: f64, t5261: f64, t1209: f64, t5231: f64, t1208: f64, t5284: f64, t10364: f64, t285: f64, t21249: f64, t281: f64, t21253: f64, t287: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22063 = t19233 * t5261;
    let t22065 = t5231 * t1209;
    let t22067 = t5284 * t1208;
    let t22068 = t10364 * t22067;
    let t22069 = t285 * t22068;
    let t22071 = t281 * t21249;
    let t22073 = t21253 * t287 * t290;
    (t22063, t22065, t22067, t22068, t22069, t22071, t22073)
}
