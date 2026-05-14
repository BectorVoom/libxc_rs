//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 830/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk830<F: Float>(t260: F, t262: F, t16578: F, t88: F, t1215: F, t1229: F, t1232: F, t1254: F, t1258: F, t1262: F, t1300: F, t1305: F, t1315: F, t1319: F, t1323: F, t155: F, t174: F, t18424: F, t18428: F, t18432: F, t18435: F, t18452: F, t331: F, t434: F, t442: F, t4605: F, t4608: F, t4620: F, t4631: F, t4635: F, t4637: F, t4697: F, t4701: F, t4719: F, t4723: F, t4730: F, t837: F) -> (F, F, F, F) {
    let t18670 = 1.0 / t260;
    let t18684 = 1.0 / t262;
    let t18708 = t16578 * t88;
    let t18709 = 1440.0 * t18708;
    let t18753 = 0.43374323531126096856e-1 * t174 * t4701 * t1315 - 0.21687161765563048428e-1 * t174 * t1300 * t4620 - t18424 + 0.13698666666666666666e0 * t174 * t4723 * t1254 + t18428 - t18432 + t18435 + 0.38527556876111295841e1 * t174 * t155 * t4605 * t4608 + 0.13218398198777742039e2 * t174 * t155 * t4635 * t4637 + 0.4406132732925914013e1 * t174 * t331 * t1258 * t1262 - 0.21309037037037037036e0 * t174 * t837 * t434 * t442 + 0.1284251895870376528e1 * t174 * t331 * t1319 * t1323 - 0.86748647062252193713e-1 * t174 * t331 * t1215 * t1305 - 0.13012297059337829057e0 * t174 * t4697 * t4730 - 0.27397333333333333333e0 * t174 * t331 * t1229 * t1232 - 0.41096e0 * t174 * t4719 * t4631 + t18452;
    (t18670, t18684, t18709, t18753)
}
