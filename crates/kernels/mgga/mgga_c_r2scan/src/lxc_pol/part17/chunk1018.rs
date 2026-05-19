//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1018/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1018<F: Float>(t12751: F, t11393: F, t11399: F, t11657: F, t11660: F, t11681: F, t11687: F, t12446: F, t12450: F, t12453: F, t12457: F, t12461: F, t12465: F, t12468: F, t12470: F, t12472: F) -> (F, F) {
    let t12752 = t12751 / F::new(4.0);
    let t12766 = -F::cast_from(0.46230515946956099004e0_f64) * t11657 - F::cast_from(0.93149212406257582492e-1_f64) * t11660 + F::cast_from(0.87327386630866483588e-2_f64) * t12446 + F::cast_from(0.43663693315433241794e-2_f64) * t12450 + F::cast_from(0.26198215989259945076e-1_f64) * t12453 - F::cast_from(0.87327386630866483588e-2_f64) * t12457 - F::cast_from(0.52396431978519890152e-1_f64) * t12461 + F::cast_from(0.13099107994629972538e-1_f64) * t12465 - t11393 + t11399 - F::cast_from(0.95219938395347901946e-2_f64) * t11681 + F::cast_from(0.46230515946956099004e0_f64) * t11687 - F::cast_from(0.10975748638225852664e0_f64) * t12468 - F::cast_from(0.17336443480108537126e0_f64) * t12470 + F::cast_from(0.32927245914677557992e0_f64) * t12472;
    (t12752, t12766)
}
