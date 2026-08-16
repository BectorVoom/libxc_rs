//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 968/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk968(t1158: f64, t21507: f64, t1326: f64, t3360: f64, t40: f64, t11262: f64, t1371: f64, t553: f64, t3379: f64, t784: f64, t1378: f64, t1971: f64) -> (f64, f64, f64, f64, f64) {
    let t29638 = t21507 * t1158;
    let t30116 = t40 * t3360 * t1326;
    let t30127 = t11262 * t1371 * t553;
    let t30129 = t784 * t3379;
    let t30131 = t30129 * t1378 * t1971;
    (t29638, t30116, t30127, t30129, t30131)
}
