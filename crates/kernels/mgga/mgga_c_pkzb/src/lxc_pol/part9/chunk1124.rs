//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1124/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1124<F: Float>(t444: F, t4803: F, t1424: F, t434: F, t4794: F, t7: F, t12584: F, t1431: F, t1425: F, t1430: F, t15: F, t82: F) -> (F, F, F, F, F, F, F) {
    let t19390 = t4803 * t444;
    let t19393 = t434 * t1424;
    let t19396 = t7 * t4794;
    let t19397 = t12584 * t1431;
    let t19400 = t1430 * t1425;
    let t19403 = t1430 * t1431;
    let t19410 = t15 * t82;
    (t19390, t19393, t19396, t19397, t19400, t19403, t19410)
}
