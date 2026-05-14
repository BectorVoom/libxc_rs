//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1243/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1243<F: Float>(t58701: F, t58714: F, t58756: F, t58782: F, t1085: F, t1094: F, t1102: F, t1220: F, t1221: F, t15008: F, t15012: F, t3284: F, t43809: F, t43834: F, t43865: F, t5103: F, t53470: F, t53494: F, t58386: F, t58390: F, t58403: F, t8426: F, t914: F) -> (F, F, F) {
    let t58784 = t58701 + t58714 + t58756 + t58782;
    let t58788 = 0.58482233974552040708e0 * t1102 * t1085 * t58784 * t1094;
    let t58791 = -64.0 / 9.0 * t15008 * t5103 + 4.0 / 3.0 * t15012 * t5103 - 1520000.0 / 243.0 * t53470 - 400.0 / 81.0 * t43809 - t1220 * t914 * t1221 * t58390 + 2.0 / 3.0 * t1220 * t914 * t3284 * t58403 - 56.0 / 9.0 * t1220 * t914 * t8426 * t58386 - 2.0 / 3.0 * t43834 - t58788 + 56.0 / 81.0 * t53494 + 32.0 / 27.0 * t43865;
    (t58784, t58788, t58791)
}
