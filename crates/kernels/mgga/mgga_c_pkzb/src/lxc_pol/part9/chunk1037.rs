//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1037/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1037<F: Float>(t12584: F, t1431: F, t1425: F, t1430: F, t15: F, t82: F, t19377: F, t19378: F, t19381: F, t19384: F, t19387: F, t19390: F, t19393: F, t19396: F, t434: F, t4784: F, t4812: F, t4820: F, t6658: F, t6659: F, t6665: F, t6679: F, t7: F, t974: F, t980: F) -> (F, F, F, F, F) {
    let t19397 = t12584 * t1431;
    let t19400 = t1430 * t1425;
    let t19403 = t1430 * t1431;
    let t19410 = t15 * t82;
    let t19417 = -10.0 / 9.0 * t19377 * t19378 + 10.0 / 9.0 * t19377 * t19381 - 10.0 / 3.0 * t6679 * t19384 - 10.0 * t6658 * t19387 + 10.0 * t6679 * t19390 - 160.0 / 9.0 * t19393 * t6659 - 10.0 / 9.0 * t19396 * t19397 - 10.0 / 9.0 * t19396 * t19400 + 10.0 / 3.0 * t6658 * t19403 - 6160.0 / 81.0 * t4784 * t974 - 40.0 / 3.0 * t434 * t6665 - 10.0 * t7 * t19410 - 40.0 / 9.0 * t980 * t4820 + 80.0 / 81.0 * t980 * t4812;
    (t19397, t19400, t19403, t19410, t19417)
}
