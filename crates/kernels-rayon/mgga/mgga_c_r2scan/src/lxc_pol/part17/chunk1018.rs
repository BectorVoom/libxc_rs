//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1018/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1018(t12751: f64, t11393: f64, t11399: f64, t11657: f64, t11660: f64, t11681: f64, t11687: f64, t12446: f64, t12450: f64, t12453: f64, t12457: f64, t12461: f64, t12465: f64, t12468: f64, t12470: f64, t12472: f64) -> (f64, f64) {
    let t12752 = t12751 / 4.0_f64;
    let t12766 = -0.46230515946956099004e0_f64 * t11657 - 0.93149212406257582492e-1_f64 * t11660 + 0.87327386630866483588e-2_f64 * t12446 + 0.43663693315433241794e-2_f64 * t12450 + 0.26198215989259945076e-1_f64 * t12453 - 0.87327386630866483588e-2_f64 * t12457 - 0.52396431978519890152e-1_f64 * t12461 + 0.13099107994629972538e-1_f64 * t12465 - t11393 + t11399 - 0.95219938395347901946e-2_f64 * t11681 + 0.46230515946956099004e0_f64 * t11687 - 0.10975748638225852664e0_f64 * t12468 - 0.17336443480108537126e0_f64 * t12470 + 0.32927245914677557992e0_f64 * t12472;
    (t12752, t12766)
}
