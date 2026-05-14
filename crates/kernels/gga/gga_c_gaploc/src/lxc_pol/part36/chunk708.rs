//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 708/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk708<F: Float>(t12704: F, t2464: F, t2684: F, t2628: F, t9817: F, t10037: F, t22256: F, t10033: F, t2617: F, t3251: F, t7810: F, t2679: F, t3243: F, t9796: F, t3255: F, t7803: F) -> (F, F, F, F, F, F, F, F) {
    let t41071 = t2684 * t2464 * t12704;
    let t41075 = t9817 * t2628;
    let t41083 = t10037 * t22256;
    let t41093 = t10033 * t2628;
    let t41133 = t7810 * t3251 * t2617;
    let t41136 = t9796 * t3243 * t2679;
    let t41139 = t9796 * t3255 * t2679;
    let t41143 = t7803 * t3243 * t2617;
    (t41071, t41075, t41083, t41093, t41133, t41136, t41139, t41143)
}
