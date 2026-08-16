//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1292/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1292(t331: f64, t50539: f64, t1076: f64, t1162: f64, t12041: f64, t13662: f64, t13678: f64, t21823: f64, t335: f64, t338: f64, t35003: f64, t35188: f64, t353: f64, t3733: f64, t3912: f64, t39475: f64, t39653: f64, t39661: f64, t46996: f64, t47008: f64, t47082: f64, t47084: f64, t47087: f64, t47143: f64, t6158: f64, t859: f64) -> f64 {
    let t50722 = t50539 * t331;
    let t50737 = t39475 * t13662 / 16.0_f64 + 7.0_f64 / 12.0_f64 * t46996 + t21823 + 7.0_f64 / 12.0_f64 * t47008 - t35003 * t859 * t353 * t1162 * t1076 / 8.0_f64 - 7.0_f64 / 48.0_f64 * t12041 * t35188 * t3733 - 7.0_f64 / 48.0_f64 * t3912 * t6158 * t50722 * t3733 + 35.0_f64 / 72.0_f64 * t39653 + 35.0_f64 / 72.0_f64 * t39661 + 7.0_f64 / 36.0_f64 * t47082 + 7.0_f64 / 24.0_f64 * t47084 - 7.0_f64 / 12.0_f64 * t47087 - t335 * t338 * t13678 * t1162 / 24.0_f64 + 7.0_f64 / 6.0_f64 * t47143;
    t50737
}
