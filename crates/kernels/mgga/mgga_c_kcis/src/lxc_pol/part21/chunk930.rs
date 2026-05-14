//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 930/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk930<F: Float>(t13391: F, t13408: F, t10819: F, t1856: F, t3620: F, t11183: F, t1240: F, t13356: F, t13367: F, t13370: F, t13373: F, t13382: F, t13389: F, t13394: F, t13399: F, t13403: F, t13406: F, t15172: F, t3638: F, t5342: F, t9572: F, t9574: F, t9576: F, t9581: F, t9600: F) -> (F, F, F) {
    let t15602 = 0.15476481481481481481e-2 * t13391;
    let t15607 = 0.15476481481481481481e-2 * t13408;
    let t15610 = t1856 * t10819;
    let t15611 = t15610 * t3620;
    let t15614 = -0.46429444444444444444e-2 * t13356 + 0.15476481481481481481e-2 * t9572 + 0.23214722222222222222e-2 * t9574 + 0.61905925925925925926e-2 * t9576 - 0.11607361111111111111e-2 * t9581 - 0.13345e0 * t3638 * t5342 + 0.11607361111111111111e-2 * t13367 - 0.17411041666666666666e-2 * t13370 - 0.34822083333333333332e-2 * t13373 - 0.15476481481481481481e-2 * t9600 - 0.41270617283950617284e-2 * t13382 - 0.23214722222222222222e-2 * t13389 + t15602 - 0.61905925925925925925e-2 * t13394 - 0.38691203703703703704e-2 * t13399 - 0.12381185185185185185e-1 * t13403 - 0.61905925925925925926e-2 * t13406 + t15607 + 0.13345e0 * t1240 * t15172 - 0.178244852896875e-2 * t11183 * t15611;
    (t15610, t15611, t15614)
}
