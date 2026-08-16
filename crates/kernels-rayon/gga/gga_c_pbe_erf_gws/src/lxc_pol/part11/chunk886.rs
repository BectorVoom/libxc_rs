//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 886/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk886(t16576: f64, t1332: f64, t35: f64, t226: f64, t7: f64, t7236: f64, t7271: f64, t4991: f64, t597: f64, t5210: f64, t735: f64, t174: f64, t177: f64, t2200: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t16577 = 192.0_f64 * t16576;
    let t16578 = t35 * t1332;
    let t16579 = 120.0_f64 * t16578;
    let t16595 = 4.0_f64 / 3.0_f64 * t226 * (-0.42777777777777777777e1_f64 * t7271 + 220.0_f64 / 81.0_f64 * t7236) * pi * t7;
    let t16621 = t4991 * t597;
    let t16666 = t5210 * t735;
    let t16704 = t174 * t2200 * t177;
    (t16577, t16578, t16579, t16595, t16621, t16666, t16704)
}
