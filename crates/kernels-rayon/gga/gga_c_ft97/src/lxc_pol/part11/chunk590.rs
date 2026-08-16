//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 590/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk590(t7798: f64, t8197: f64, t348: f64, t1559: f64, t432: f64, t3187: f64, t1902: f64, t1557: f64, t487: f64, t492: f64, t3193: f64, t1586: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8198 = t7798 + t8197;
    let t8199 = t348 * t8198;
    let t8205 = t1559 * t432;
    let t8206 = t3187 * t8205;
    let t8207 = t1902 * t8206;
    let t8210 = t487 * t1557;
    let t8211 = t1559 * t492;
    let t8212 = t8210 * t8211;
    let t8213 = t3193 * t8212;
    let t8216 = t355 * t1586;
    (t8198, t8199, t8205, t8206, t8207, t8210, t8211, t8212, t8213, t8216)
}
