//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 957/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk957<F: Float>(t3645: F, t547: F, t1603: F, t862: F, t865: F, t1659: F, t3892: F, t3035: F, t3923: F, t545: F, t1658: F, t316: F, t449: F, t879: F) -> (F, F, F, F, F) {
    let t15253 = t3645 * t547;
    let t15259 = t862 * t1603 * t865;
    let t15262 = t3892 * t1659;
    let t15265 = t3035 * t545 * t3923;
    let t15276 = t316 * t449 * t1658 * t879;
    (t15253, t15259, t15262, t15265, t15276)
}
