//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 624/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk624(t252: f64, t5385: f64, t245: f64, t713: f64, t1697: f64, t212: f64, t22: f64, t219: f64, t5063: f64, t1923: f64, t247: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5387 = 8.0_f64 / 81.0_f64 * t252 * t5385;
    let t5390 = t245 * t713;
    let t5399 = 1.0_f64 / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5401 = t219 * t5063;
    let t5420 = t247 * t1923;
    let t5421 = t24 * t5420;
    (t5387, t5390, t5399, t5400, t5401, t5420, t5421)
}
