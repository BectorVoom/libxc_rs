//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 797/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk797<F: Float>(t2554: F, t7064: F, t9637: F, t12608: F, t2549: F, t12612: F, t2562: F, t28197: F, t883: F, t943: F, t12623: F, t10053: F, t2558: F) -> (F, F, F, F, F, F) {
    let t40699 = t7064 * t9637 * t2554;
    let t40744 = t2549 * t12608;
    let t40746 = t2549 * t12612;
    let t40750 = t943 * t2562 * t883 * t28197;
    let t40752 = t2549 * t12623;
    let t40758 = t943 * t10053 * t2558;
    (t40699, t40744, t40746, t40750, t40752, t40758)
}
