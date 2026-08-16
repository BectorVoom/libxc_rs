//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 808/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk808(t3142: f64, t5319: f64, t9247: f64, t9246: f64, t3137: f64, t1461: f64, t2993: f64, t1038: f64, t5972: f64, t3712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t9248 = t5319 * t3142;
    let t9249 = t9247 * t9248;
    let t9250 = t9246 * t9249;
    let t9252 = t3137 * pi;
    let t9253 = t1461 * t9252;
    let t9254 = t2993 * t9253;
    let t9255 = t1038 * t5972;
    let t9256 = t3712 * t9255;
    (t9249, t9250, t9252, t9253, t9254, t9255, t9256)
}
