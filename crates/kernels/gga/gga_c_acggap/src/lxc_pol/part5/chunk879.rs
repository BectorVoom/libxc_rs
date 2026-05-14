//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 879/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk879<F: Float>(t1658: F, t316: F, t449: F, t879: F, t5368: F, t852: F, t119: F, t5299: F, t5360: F, t880: F, t3054: F, t545: F, t865: F, t5332: F, t857: F, t3874: F, t556: F) -> (F, F, F, F, F, F, F) {
    let t15276 = t316 * t449 * t1658 * t879;
    let t15278 = t852 * t5368;
    let t15285 = t119 * t5299;
    let t15290 = t5360 * t880;
    let t15293 = t3054 * t545 * t865;
    let t15295 = t857 * t5332;
    let t15297 = t3874 * t556;
    (t15276, t15278, t15285, t15290, t15293, t15295, t15297)
}
