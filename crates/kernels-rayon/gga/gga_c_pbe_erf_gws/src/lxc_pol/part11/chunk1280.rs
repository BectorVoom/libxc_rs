//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1280/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1280(t1076: f64, t328: f64, t1115: f64, t1144: f64, t12111: f64, t13174: f64, t13227: f64, t13613: f64, t2118: f64, t27077: f64, t27079: f64, t3052: f64, t3207: f64, t335: f64, t338: f64, t3733: f64, t3912: f64, t3913: f64, t43375: f64, t43740: f64, t44063: f64, t44158: f64, t46635: f64, t46637: f64, t8713: f64, t9283: f64, t9838: f64) -> (f64, f64) {
    let t50539 = t1076 * t328;
    let t50544 = 7.0_f64 / 12.0_f64 * t44158 - 455.0_f64 / 324.0_f64 * t27077 - 455.0_f64 / 324.0_f64 * t27079 - t13174 * t3052 / 12.0_f64 - t43740 * t3733 / 16.0_f64 + t3913 * t12111 / 8.0_f64 - t1115 * t44063 / 4.0_f64 - t1115 * t43375 / 8.0_f64 - 3.0_f64 / 4.0_f64 * t3207 * t9283 * t8713 * t13227 - t335 * t338 * t1144 * t13613 / 4.0_f64 + 7.0_f64 / 72.0_f64 * t46635 + 7.0_f64 / 24.0_f64 * t46637 + t3912 * t2118 * t50539 * t9838 / 8.0_f64;
    (t50539, t50544)
}
