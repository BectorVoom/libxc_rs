//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3196/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3196(t1227: f64, t13969: f64, t19067: f64, t1222: f64, t18297: f64, t18982: f64, t18947: f64, t3506: f64, t11719: f64, t18302: f64, t11546: f64, t1174: f64, t11814: f64, t15625: f64, t15761: f64, t18300: f64, t3440: f64, t3507: f64, t44725: f64, t44863: f64, t44906: f64, t45030: f64, t45178: f64, t4582: f64, t5005: f64, t53267: f64, t6221: f64, t63378: f64, t63386: f64, t63394: f64) -> f64 {
    let t66406 = t1227 * t13969 * t19067;
    let t66408 = t18297 * t1222;
    let t66410 = t18982 * t1222;
    let t66413 = t3506 * t13969 * t18947;
    let t66437 = t11719 * t13969 * t18302;
    let t66442 = -t5005 * t15761 / 2304.0_f64 + t11814 * t6221 / 3072.0_f64 + 5.0_f64 / 10368.0_f64 * t66406 - t66408 / 216.0_f64 + 19.0_f64 / 1296.0_f64 * t66410 + t66413 / 576.0_f64 + t1174 * t3440 * t63394 / 108.0_f64 + t1174 * t3440 * t63386 / 36.0_f64 + t11719 * t4582 * t18300 * t44906 / 512.0_f64 + t44863 * t4582 * t18300 * t44725 * t3507 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t45030 * t4582 * t18300 * t15625 + t45178 / 648.0_f64 - t53267 / 3456.0_f64 + t66437 / 384.0_f64 - 7.0_f64 / 54.0_f64 * t1174 * t11546 * t63378;
    t66442
}
