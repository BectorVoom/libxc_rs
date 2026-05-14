//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 930/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk930<F: Float>(t11514: F, t2171: F, t2345: F, t6229: F, t11464: F, t3140: F, t3235: F, t3752: F, t810: F, t1123: F, t2255: F, t814: F, t3258: F, t2257: F, t3781: F, t11492: F, t11494: F, t11495: F, t11497: F, t11502: F, t11506: F, t11513: F, t2253: F, t2277: F, t2312: F, t2343: F, t3247: F, t6579: F, t902: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11516 = t2345 * t11514 * t2171;
    let t11519 = 35.0 / 432.0 * t6229;
    let t11521 = t3235 * t11464 * t3140;
    let t11524 = t3752 * t810;
    let t11525 = t1123 * t11524;
    let t11526 = t2255 * t11525;
    let t11529 = t3752 * t814;
    let t11530 = t3258 * t11529;
    let t11531 = t2255 * t11530;
    let t11534 = t3781 * t2257;
    let t11535 = t2255 * t11534;
    let t11538 = -t11492 + t11494 + 7.0 / 2304.0 * t11495 + 7.0 / 2304.0 * t11497 + t2277 * t11502 / 384.0 + t902 * t11506 / 1536.0 + t11513 + t2343 * t11516 / 384.0 - t11519 + t3247 * t11521 / 512.0 + 5.0 / 192.0 * t6579 * t11526 + t2312 * t11531 / 192.0 - t2253 * t11535 / 768.0;
    (t11516, t11519, t11521, t11525, t11526, t11530, t11531, t11534, t11535, t11538)
}
