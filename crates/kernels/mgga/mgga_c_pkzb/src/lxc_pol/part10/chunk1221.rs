//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1221/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1221<F: Float>(t1220: F, t6433: F, t154: F, t3026: F, t385: F, t6446: F, t6452: F, t2347: F, t7945: F, t6448: F, t1167: F, t19023: F, t3214: F, t6467: F, t1229: F, t17955: F, t918: F) -> (F, F, F, F, F, F, F, F) {
    let t23313 = t1220 * t6433;
    let t23317 = t385 * t154 * t6446 * t3026;
    let t23319 = t1220 * t6452;
    let t23325 = t385 * t154 * t2347 * t7945;
    let t23331 = t1220 * t6448;
    let t23338 = t385 * t154 * t19023 * t1167;
    let t23340 = t3214 * t6467;
    let t23345 = t918 * t17955 * t1229;
    (t23313, t23317, t23319, t23325, t23331, t23338, t23340, t23345)
}
