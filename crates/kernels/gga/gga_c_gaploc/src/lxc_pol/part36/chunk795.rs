//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 795/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk795<F: Float>(t12557: F, t2518: F, t135: F, t1691: F, t458: F, t5337: F, t9105: F, t15667: F, t39636: F, t1233: F, t15665: F, t15672: F, t39635: F, t92: F) -> (F, F, F, F) {
    let t40614 = t2518 * t12557;
    let t40620 = t9105 * t5337 * M_PI * t1691 * t135 * t458;
    let t40622 = t39636 * t15667;
    let t40627 = t15672 * t1233 * t39635 * t15665 * t92;
    (t40614, t40620, t40622, t40627)
}
