//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 687/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk687<F: Float>(t1233: F, t15665: F, t15672: F, t39635: F, t92: F, t39644: F, t5345: F, t5348: F, t1692: F, t2519: F, t12568: F, t716: F, t1902: F, t883: F, t12691: F, t2464: F, t825: F) -> (F, F, F, F, F, F) {
    let t40627 = t15672 * t1233 * t39635 * t15665 * t92;
    let t40630 = t5345 * t39644 * t5348;
    let t40632 = t1692 * t2519;
    let t40634 = t12568 * t716;
    let t40820 = t883 * t1902;
    let t41060 = t825 * t2464 * t12691;
    (t40627, t40630, t40632, t40634, t40820, t41060)
}
