//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 642/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk642<F: Float>(t1510: F, t410: F, t4911: F, t89: F, t36: F, t409: F, t1385: F, t732: F, t1380: F, t453: F, t4811: F, t234: F, t1409: F, t1497: F, t454: F, t452: F) -> (F, F, F, F, F, F, F, F) {
    let t4978 = t410 * t1510;
    let t4979 = 12.0 * t4978;
    let t4980 = t4911 * t89;
    let t4981 = 24.0 * t4980;
    let t4982 = t36 * t409;
    let t4983 = t4982 * t89;
    let t4987 = t732 * t1385;
    let t4990 = t1380 * t4811 * t453;
    let t4991 = t234 * t4990;
    let t4992 = 0.35089341735807877242e1 * t4991;
    let t4994 = t1497 * t1409 * t454;
    let t4995 = t234 * t4994;
    let t4996 = 0.35089341735807877242e1 * t4995;
    let t4997 = t1380 * t452;
    (t4979, t4981, t4982, t4983, t4987, t4992, t4996, t4997)
}
