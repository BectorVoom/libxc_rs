//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 917/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk917<F: Float>(t3332: F, t9380: F, t6165: F, t3308: F, t1592: F, t269: F, t3177: F, t1060: F, t783: F, t106: F, t3052: F, t97: F) -> (F, F, F, F, F, F, F) {
    let t12543 = t3332 * t9380;
    let t12544 = t6165 * t12543;
    let t12547 = t3308 * t9380;
    let t12548 = t1592 * t12547;
    let t12550 = t3177 * t269;
    let t12552 = t783 * t12550 * t1060;
    let t12567 = t97 * t106 * t3052;
    (t12543, t12544, t12547, t12548, t12550, t12552, t12567)
}
