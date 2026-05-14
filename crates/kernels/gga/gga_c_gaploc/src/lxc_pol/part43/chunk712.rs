//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 712/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk712<F: Float>(t12691: F, t2464: F, t825: F, t12663: F, t549: F, t6111: F, t12704: F, t2684: F, t2628: F, t9817: F, t10037: F, t22256: F, t10033: F, t2617: F, t3251: F, t7810: F) -> (F, F, F, F, F, F, F) {
    let t41060 = t825 * t2464 * t12691;
    let t41068 = t6111 * t549 * t12663;
    let t41071 = t2684 * t2464 * t12704;
    let t41075 = t9817 * t2628;
    let t41083 = t10037 * t22256;
    let t41093 = t10033 * t2628;
    let t41133 = t7810 * t3251 * t2617;
    (t41060, t41068, t41071, t41075, t41083, t41093, t41133)
}
