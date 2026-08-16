//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 986/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk986(t1383: f64, t169: f64, t3689: f64, t3373: f64, t39: f64, t1477: f64, t1480: f64, t3379: f64, t551: f64, t142: f64, t985: f64, t10207: f64, t751: f64) -> (f64, f64, f64, f64, f64) {
    let t34254 = t169 * t3689 * t1383;
    let t34274 = t39 * t3373;
    let t34300 = t1477 * t3379 * t551 * t1480;
    let t34302 = t985 * t142;
    let t34326 = t751 * t10207;
    (t34254, t34274, t34300, t34302, t34326)
}
