//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 877/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk877<F: Float>(t11554: F, t795: F, t3276: F, t3275: F, t2867: F, t792: F, t158: F, t955: F, t874: F, t3446: F, t3447: F, t122: F, t3434: F, t3437: F, t1103: F, t2461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11555 = t11554 * t795;
    let t11556 = t3276 * t11555;
    let t11557 = t3275 * t11556;
    let t11558 = 5.0 / 16.0 * t11557;
    let t11559 = t2867 * t792;
    let t11560 = t3276 * t11559;
    let t11561 = t3275 * t11560;
    let t11562 = 5.0 / 16.0 * t11561;
    let t11563 = t158 * t955;
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    (t11555, t11556, t11557, t11558, t11559, t11560, t11561, t11562, t11563, t11564, t11566, t11568, t11570, t11572)
}
