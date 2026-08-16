//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1028/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1028(t13391: f64, t13408: f64, t10819: f64, t1856: f64, t3620: f64, t11183: f64, t1240: f64, t13356: f64, t13367: f64, t13370: f64, t13373: f64, t13382: f64, t13389: f64, t13394: f64, t13399: f64, t13403: f64, t13406: f64, t15172: f64, t3638: f64, t5342: f64, t9572: f64, t9574: f64, t9576: f64, t9581: f64, t9600: f64) -> (f64, f64, f64) {
    let t15602 = 0.15476481481481481481e-2_f64 * t13391;
    let t15607 = 0.15476481481481481481e-2_f64 * t13408;
    let t15610 = t1856 * t10819;
    let t15611 = t15610 * t3620;
    let t15614 = -0.46429444444444444444e-2_f64 * t13356 + 0.15476481481481481481e-2_f64 * t9572 + 0.23214722222222222222e-2_f64 * t9574 + 0.61905925925925925926e-2_f64 * t9576 - 0.11607361111111111111e-2_f64 * t9581 - 0.13345e0_f64 * t3638 * t5342 + 0.11607361111111111111e-2_f64 * t13367 - 0.17411041666666666666e-2_f64 * t13370 - 0.34822083333333333332e-2_f64 * t13373 - 0.15476481481481481481e-2_f64 * t9600 - 0.41270617283950617284e-2_f64 * t13382 - 0.23214722222222222222e-2_f64 * t13389 + t15602 - 0.61905925925925925925e-2_f64 * t13394 - 0.38691203703703703704e-2_f64 * t13399 - 0.12381185185185185185e-1_f64 * t13403 - 0.61905925925925925926e-2_f64 * t13406 + t15607 + 0.13345e0_f64 * t1240 * t15172 - 0.178244852896875e-2_f64 * t11183 * t15611;
    (t15610, t15611, t15614)
}
