//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 928/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk928<F: Float>(t154: F, t6182: F, t1038: F, t15489: F, t9863: F, t22581: F, t8676: F, t1084: F, t26312: F, t2536: F, t2763: F, t6188: F, t22783: F, t7556: F, t966: F, t9864: F) -> (F, F, F, F, F, F, F, F) {
    let t28920 = t154 * t6182;
    let t28924 = t9863 * t1038 * t15489;
    let t29006 = t8676 * t22581;
    let t29033 = t1084 * t26312;
    let t29070 = t1038 * t2763 * t2536;
    let t29108 = t154 * t6188;
    let t29207 = t8676 * t22783;
    let t29228 = t7556 * t966 * t9864;
    (t28920, t28924, t29006, t29033, t29070, t29108, t29207, t29228)
}
