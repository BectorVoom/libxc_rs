//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1126/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1126<F: Float>(t10928: F, t122: F, t3434: F, t874: F, t955: F, t1561: F, t3274: F, t97: F, t11582: F, t38248: F, t38249: F, t5086: F) -> (F, F, F, F) {
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    let t40574 = t97 * t3274 * t1561;
    let t40587 = t38248 * t11582 * t38249;
    let t40594 = t97 * t3274 * t5086;
    (t40564, t40574, t40587, t40594)
}
