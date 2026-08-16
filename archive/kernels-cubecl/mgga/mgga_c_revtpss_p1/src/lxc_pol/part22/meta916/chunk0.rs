//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3125/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3125<F: Float>(t3057: F, t4930: F, t15886: F, t378: F, t3046: F, t1072: F, t1647: F, t3259: F, t1071: F, t15669: F, t15654: F, t12050: F, t15907: F) -> (F, F, F, F, F, F, F, F) {
    let t55413 = t3057 * t4930;
    let t55416 = t15886 * t378;
    let t55421 = t3046 * t4930;
    let t55458 = t3057 * t1072;
    let t55461 = t1647 * t3259;
    let t55464 = t15669 * t1071;
    let t55475 = t15654 * t1071;
    let t55499 = t15907 * t12050;
    (t55413, t55416, t55421, t55458, t55461, t55464, t55475, t55499)
}
