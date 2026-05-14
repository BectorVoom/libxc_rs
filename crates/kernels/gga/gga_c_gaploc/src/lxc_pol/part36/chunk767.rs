//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 767/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk767<F: Float>(t8411: F, t9327: F, t10556: F, t1415: F, t9321: F, t12792: F, t12793: F, t12971: F, t12972: F, t12975: F, t12982: F, t1339: F, t1537: F, t1589: F, t1628: F, t188: F, t189: F, t193: F, t40455: F, t40458: F, t42123: F, t42309: F, t42312: F, t42315: F, t42316: F, t42340: F, t42341: F, t42350: F, t42354: F, t524: F, t541: F, t557: F, t568: F, t574: F, t590: F, t597: F, t600: F) -> (F,) {
    let t42356 = 0.10725146985555128001e1 * t8411 * t9327;
    let t42359 = 0.42900587942220512003e1 * t1415 * t10556 * t9321;
    let t42360 = -t42309 - t42312 - t42315 - 0.29792074959875355558e-1 * t42316 + 0.23005755572352449806e1 * t597 * t568 * t600 * t42123 + 0.30674340763136599741e1 * t597 * t1628 * t12975 - 0.23833659967900284446e0 * t557 * t1589 * t12793 + 0.23833659967900284446e0 * t12972 * t541 - 0.30674340763136599741e1 * t574 * t1628 * t12982 + 0.35750489951850426669e0 * t524 * t12971 * t193 + 0.35750489951850426669e0 * t188 * t189 * t42123 * t193 + t42340 + t42341 - 0.51123901271894332901e0 * t40455 + 0.38342925953920749676e0 * t40458 - 0.51123901271894332902e0 * t1537 * t1339 * t12792 * t590 - t42350 + t42354 + t42356 - t42359;
    (t42360,)
}
