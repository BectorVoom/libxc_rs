//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1290/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1290(t1118: f64, t1144: f64, t13110: f64, t13639: f64, t13678: f64, t2401: f64, t2409: f64, t3067: f64, t3207: f64, t335: f64, t338: f64, t36290: f64, t3703: f64, t3721: f64, t3737: f64, t3887: f64, t3896: f64, t3907: f64, t3917: f64, t4386: f64, t43983: f64, t46759: f64, t46858: f64, t46870: f64, t46872: f64, t844: f64, t8629: f64, t8793: f64, t9899: f64) -> f64 {
    let t50681 = t8629 * t4386 * t1144 * t13110 / 4.0_f64 + t8793 * t43983 / 4.0_f64 - 7.0_f64 / 24.0_f64 * t46759 + 3.0_f64 / 8.0_f64 * t2401 * t338 * t3907 * t3737 - t844 * t338 * t13678 * t1118 / 12.0_f64 - t844 * t338 * t1144 * t13639 / 12.0_f64 - t844 * t338 * t3907 * t3896 / 8.0_f64 - 35.0_f64 / 72.0_f64 * t36290 - t3917 * t9899 / 16.0_f64 - 7.0_f64 / 72.0_f64 * t46858 - 7.0_f64 / 24.0_f64 * t46870 + 3.0_f64 / 4.0_f64 * t3207 * t2409 * t3067 * t3703 * t3721 - 7.0_f64 / 6.0_f64 * t46872 - t335 * t338 * t3907 * t3887 / 16.0_f64;
    t50681
}
