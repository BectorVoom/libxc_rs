//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1268/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1268<F: Float>(t22744: F, t8844: F, t2294: F, t2582: F, t9147: F, t6118: F, t9258: F, t1616: F, t3190: F, t5103: F, t785: F, t3056: F, t551: F, t574: F, t6343: F, t1592: F, t3090: F) -> (F, F, F, F, F, F) {
    let t29405 = t22744 * t8844;
    let t29409 = t2582 * t2294 * t9147;
    let t29411 = t6118 * t9258;
    let t29415 = t5103 * t785 * t1616 * t3190;
    let t29443 = t574 * t551 * t6343 * t3056;
    let t29447 = t1592 * t551 * t6343 * t3090;
    (t29405, t29409, t29411, t29415, t29443, t29447)
}
