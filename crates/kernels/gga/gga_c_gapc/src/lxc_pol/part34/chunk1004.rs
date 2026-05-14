//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1004/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1004<F: Float>(t15553: F, t15555: F, t33287: F, t33158: F, t3402: F, t3408: F, t1084: F, t11428: F, t11927: F, t1461: F, t818: F, t15507: F, t8: F, t29867: F, t332: F, t6: F, t7875: F) -> (F, F, F, F, F) {
    let t33510 = t15553 * t33287 * t15555;
    let t33513 = t3402 * t33158 * t3408;
    let t33518 = t1084 * t1461 * t11428 * t818 * t11927;
    let t33521 = 1.0 / t8 / t15507;
    let t33527 = t7875 * t332 * t6 * t29867;
    (t33510, t33513, t33518, t33521, t33527)
}
