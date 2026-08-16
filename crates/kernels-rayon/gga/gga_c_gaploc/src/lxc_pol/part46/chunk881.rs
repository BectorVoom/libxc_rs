//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 881/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk881(t12792: f64, t12793: f64, t12971: f64, t12972: f64, t12975: f64, t12982: f64, t1339: f64, t1537: f64, t1589: f64, t1628: f64, t188: f64, t189: f64, t193: f64, t40455: f64, t40458: f64, t42123: f64, t42309: f64, t42312: f64, t42315: f64, t42316: f64, t42340: f64, t42341: f64, t42350: f64, t42354: f64, t42356: f64, t42359: f64, t524: f64, t541: f64, t557: f64, t568: f64, t574: f64, t590: f64, t597: f64, t600: f64) -> f64 {
    let t42360 = -t42309 - t42312 - t42315 - 0.29792074959875355558e-1_f64 * t42316 + 0.23005755572352449806e1_f64 * t597 * t568 * t600 * t42123 + 0.30674340763136599741e1_f64 * t597 * t1628 * t12975 - 0.23833659967900284446e0_f64 * t557 * t1589 * t12793 + 0.23833659967900284446e0_f64 * t12972 * t541 - 0.30674340763136599741e1_f64 * t574 * t1628 * t12982 + 0.35750489951850426669e0_f64 * t524 * t12971 * t193 + 0.35750489951850426669e0_f64 * t188 * t189 * t42123 * t193 + t42340 + t42341 - 0.51123901271894332901e0_f64 * t40455 + 0.38342925953920749676e0_f64 * t40458 - 0.51123901271894332902e0_f64 * t1537 * t1339 * t12792 * t590 - t42350 + t42354 + t42356 - t42359;
    t42360
}
