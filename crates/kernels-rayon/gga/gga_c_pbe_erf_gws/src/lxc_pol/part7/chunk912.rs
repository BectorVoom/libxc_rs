//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 912/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk912(t5102: f64, t633: f64, t1672: f64, t1725: f64, t211: f64, t1406: f64, t1820: f64, t1885: f64, t5299: f64, t5292: f64, t9: f64, t5295: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t17163 = t633 * t5102;
    let t17164 = 16.0_f64 / 45.0_f64 * t17163;
    let t17166 = t211 * t1672 * t1725;
    let t17167 = 8.0_f64 / 45.0_f64 * t17166;
    let t17171 = 16.0_f64 / 5.0_f64 * t1820 * t1885 * t5299 * t1406;
    let t17172 = t9 * t5292;
    let t17174 = t587 * t17172 * t5295;
    (t17164, t17167, t17171, t17174)
}
