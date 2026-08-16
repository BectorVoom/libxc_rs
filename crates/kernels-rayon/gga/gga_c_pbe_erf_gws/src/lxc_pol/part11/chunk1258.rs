//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1258/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1258(t2210: f64, t49955: f64, t858: f64, t884: f64, t19561: f64, t21621: f64, t21623: f64, t49063: f64, t860: f64, t20137: f64, t2300: f64, t3247: f64, t3808: f64, t38506: f64, t45852: f64, t49064: f64, t49092: f64, t49842: f64, t49950: f64, t49952: f64, t49954: f64, t902: f64, t904: f64, t905: f64, t929: f64, t9665: f64) -> (f64, f64, f64) {
    let t49963 = 3.0_f64 / 16.0_f64 * t884 * t2210 * t858 * t49955;
    let t49980 = t21621 * t49063 * t19561 * t21623 * t860 / 96.0_f64;
    let t49981 = t49950 + t49952 - 119.0_f64 / 1152.0_f64 * t38506 + t49954 + 5.0_f64 / 256.0_f64 * t929 * t2300 * t904 * t49955 + t49963 - 7.0_f64 / 576.0_f64 * t45852 + t902 * t905 * t3808 * t49092 / 256.0_f64 - 3.0_f64 / 32.0_f64 * t3247 * t9665 * t49842 + t902 * t905 * t49064 * t20137 / 192.0_f64 + t49980;
    (t49963, t49980, t49981)
}
