//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 830/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk830(t13173: f64, t826: f64, t1161: f64, t3703: f64, t2376: f64, t2409: f64, t1105: f64, t3717: f64) -> (f64, f64, f64, f64) {
    let t13174 = t13173 * t826;
    let t13182 = t3703 * t1161;
    let t13184 = t2409 * t2376 * t13182;
    let t13187 = t1105 * t3717;
    (t13174, t13182, t13184, t13187)
}
