//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 877/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk877(t13137: f64, t13201: f64, t13638: f64, t13698: f64, t945: f64, t321: f64, t1105: f64, t3929: f64, t804: f64, t13220: f64, t382: f64, t1168: f64, t3717: f64, param_gamma: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13700 = t13137 + t13201 + t13638 + t13698;
    let t13701 = t13700 * t945;
    let t13702 = t321 * t13701;
    let t13704 = t804 * t3929 * t1105;
    let t13707 = param_gamma * t13220;
    let t13708 = t13707 * t382;
    let t13711 = t804 * t1168 * t3717;
    (t13700, t13701, t13702, t13704, t13707, t13708, t13711)
}
