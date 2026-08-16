//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1056/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1056<F: Float>(t10972: F, t37373: F, t37369: F, t10977: F, t10981: F, t37372: F, t122: F, t607: F, t10928: F, t3434: F, t874: F, t1266: F, t550: F) -> (F, F, F, F, F) {
    let t37458 = t37373 * t10972;
    let t37460 = t37369 * t10972;
    let t37463 = t37372 * t10977 * t10981;
    let t37465 = t607 * t122;
    let t37468 = t3434 * t10928 * t37465 * t874;
    let t37470 = t550 * t1266;
    (t37458, t37460, t37463, t37468, t37470)
}
