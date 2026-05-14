//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 703/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk703<F: Float>(t1692: F, t2519: F, t12568: F, t716: F, t2554: F, t7064: F, t9642: F, t2558: F, t28152: F, t9647: F, t9637: F, t12608: F, t2549: F, t12612: F, t2562: F, t28197: F, t883: F, t943: F) -> (F, F, F, F, F, F, F, F) {
    let t40632 = t1692 * t2519;
    let t40634 = t12568 * t716;
    let t40693 = t7064 * t9642 * t2554;
    let t40696 = t9647 * t28152 * t2558;
    let t40699 = t7064 * t9637 * t2554;
    let t40744 = t2549 * t12608;
    let t40746 = t2549 * t12612;
    let t40750 = t943 * t2562 * t883 * t28197;
    (t40632, t40634, t40693, t40696, t40699, t40744, t40746, t40750)
}
