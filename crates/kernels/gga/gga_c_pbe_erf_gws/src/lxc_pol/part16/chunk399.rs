//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 399/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk399<F: Float>(t1383: F, t169: F, t289: F, t274: F, t39: F, t532: F, t745: F, t1216: F, t1319: F, t1322: F, t470: F, t1314: F, t449: F, t456: F, t427: F, t75: F) -> (F, F, F, F, F, F, F, F) {
    let t1386 = 0.31835665774679373271e-1 * t169 * t289 * t1383;
    let t1388 = 0.3199504064530762818e0 * t39 * t274;
    let t1389 = t532 * t745;
    let t1392 = t1319 * t1216 * t1322;
    let t1393 = t470 * t1392;
    let t1394 = 0.17315755899375863299e2 * t1393;
    let t1396 = t449 * t1314 * t456;
    let t1397 = t470 * t1396;
    let t1398 = 0.58482233974552040708e0 * t1397;
    let t1399 = t427 * t75;
    (t1386, t1388, t1389, t1392, t1394, t1396, t1398, t1399)
}
