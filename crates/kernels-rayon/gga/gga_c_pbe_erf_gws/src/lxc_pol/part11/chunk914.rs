//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 914/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk914(t18708: f64, t1215: f64, t1229: f64, t1232: f64, t1254: f64, t1258: f64, t1262: f64, t1300: f64, t1305: f64, t1315: f64, t1319: f64, t1323: f64, t155: f64, t174: f64, t18424: f64, t18428: f64, t18432: f64, t18435: f64, t18452: f64, t331: f64, t434: f64, t442: f64, t4605: f64, t4608: f64, t4620: f64, t4631: f64, t4635: f64, t4637: f64, t4697: f64, t4701: f64, t4719: f64, t4723: f64, t4730: f64, t837: f64) -> (f64, f64) {
    let t18709 = 1440.0_f64 * t18708;
    let t18753 = 0.43374323531126096856e-1_f64 * t174 * t4701 * t1315 - 0.21687161765563048428e-1_f64 * t174 * t1300 * t4620 - t18424 + 0.13698666666666666666e0_f64 * t174 * t4723 * t1254 + t18428 - t18432 + t18435 + 0.38527556876111295841e1_f64 * t174 * t155 * t4605 * t4608 + 0.13218398198777742039e2_f64 * t174 * t155 * t4635 * t4637 + 0.4406132732925914013e1_f64 * t174 * t331 * t1258 * t1262 - 0.21309037037037037036e0_f64 * t174 * t837 * t434 * t442 + 0.1284251895870376528e1_f64 * t174 * t331 * t1319 * t1323 - 0.86748647062252193713e-1_f64 * t174 * t331 * t1215 * t1305 - 0.13012297059337829057e0_f64 * t174 * t4697 * t4730 - 0.27397333333333333333e0_f64 * t174 * t331 * t1229 * t1232 - 0.41096e0_f64 * t174 * t4719 * t4631 + t18452;
    (t18709, t18753)
}
