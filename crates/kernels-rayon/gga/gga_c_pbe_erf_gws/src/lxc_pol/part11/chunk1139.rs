//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1139/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1139(t41524: f64, t41562: f64, t41570: f64, t41573: f64, t12651: f64, t2615: f64, t47391: f64, t5008: f64, t587: f64, t590: f64, t12572: f64, t18309: f64, t18311: f64, t18315: f64, t18318: f64, t34538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48130 = 64.0_f64 / 45.0_f64 * t41524;
    let t48132 = 32.0_f64 / 45.0_f64 * t41562;
    let t48133 = 32.0_f64 / 135.0_f64 * t41570;
    let t48134 = 256.0_f64 / 243.0_f64 * t41573;
    let t48136 = 16.0_f64 / 5.0_f64 * t2615 * t12651;
    let t48140 = 32.0_f64 / 15.0_f64 * t587 * t590 * t5008 * t47391;
    let t48142 = 32.0_f64 / 15.0_f64 * t2615 * t12572;
    let t48143 = t18309 + t18311 - t18315 - t18318 + t48130 - 4.0_f64 / 9.0_f64 * t34538 + t48132 + t48133 + t48134 + t48136 - t48140 + t48142;
    (t48130, t48132, t48133, t48134, t48136, t48140, t48142, t48143)
}
