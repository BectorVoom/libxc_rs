//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 920/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk920<F: Float>(t12737: F, t12414: F, t3465: F, t10610: F, t12383: F, t3472: F, t3275: F, t1149: F, t2995: F, t12056: F, t3262: F, t3574: F, t8601: F, t11393: F, t11399: F, t11657: F, t11660: F, t11681: F, t11687: F, t12446: F, t12450: F, t12453: F, t12457: F, t12461: F, t12465: F, t12468: F, t12470: F, t12472: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12738 = 5.0 / 8.0 * t12737;
    let t12739 = t3465 * t12414;
    let t12740 = t10610 * t12739;
    let t12741 = 3.0 / 2.0 * t12740;
    let t12742 = t3472 * t12383;
    let t12743 = t3275 * t12742;
    let t12744 = 5.0 / 8.0 * t12743;
    let t12745 = t2995 * t1149;
    let t12747 = t3262 * t12056 * t3574;
    let t12748 = 3.0 / 2.0 * t12747;
    let t12751 = t3275 * t3465 * t8601;
    let t12752 = t12751 / 4.0;
    let t12766 = -0.46230515946956099004e0 * t11657 - 0.93149212406257582492e-1 * t11660 + 0.87327386630866483588e-2 * t12446 + 0.43663693315433241794e-2 * t12450 + 0.26198215989259945076e-1 * t12453 - 0.87327386630866483588e-2 * t12457 - 0.52396431978519890152e-1 * t12461 + 0.13099107994629972538e-1 * t12465 - t11393 + t11399 - 0.95219938395347901946e-2 * t11681 + 0.46230515946956099004e0 * t11687 - 0.10975748638225852664e0 * t12468 - 0.17336443480108537126e0 * t12470 + 0.32927245914677557992e0 * t12472;
    (t12738, t12739, t12741, t12742, t12744, t12745, t12748, t12752, t12766)
}
