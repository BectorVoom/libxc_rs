//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1106/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1106(t40718: f64, t1017: f64, t1885: f64, t40571: f64, t587: f64, t17182: f64, t17183: f64, t47391: f64, t10383: f64, t3443: f64, t1620: f64, t1621: f64, t31503: f64, t3390: f64) -> (f64, f64, f64, f64, f64) {
    let t47707 = 32.0_f64 / 27.0_f64 * t40718;
    let t47711 = 16.0_f64 / 15.0_f64 * t587 * t1885 * t40571 * t1017;
    let t47715 = 352.0_f64 / 243.0_f64 * t587 * t17182 * t17183 * t47391;
    let t47719 = 24.0_f64 / 5.0_f64 * t587 * t1885 * t10383 * t3443;
    let t47723 = 16.0_f64 / 5.0_f64 * t1620 * t1621 * t31503 * t3390;
    (t47707, t47711, t47715, t47719, t47723)
}
