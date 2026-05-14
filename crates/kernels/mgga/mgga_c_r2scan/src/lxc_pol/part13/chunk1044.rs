//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1044/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1044<F: Float>(t3308: F, t37961: F, t7368: F, t10776: F, t7429: F, t10781: F, t7505: F, t38028: F, t38033: F, t39992: F, t39996: F, t39998: F, t40001: F, t40004: F, t40007: F, t40011: F) -> (F,) {
    let t40016 = t37961 * t3308 * t7368;
    let t40019 = t10776 * t3308 * t7429;
    let t40021 = t10781 * t7505;
    let t40023 = 0.26198215989259945075e-1 * t39992 + t39996 + 0.13099107994629972538e-1 * t39998 + 0.13972381860938637373e0 * t40001 + 0.13099107994629972538e0 * t40004 - 0.2600466522016280569e0 * t40007 + 0.65495539973149862688e-2 * t40011 + 0.23804984598836975486e-2 * t38028 + 0.31147743054556651236e-1 * t38033 - 0.2600466522016280569e0 * t40016 + 0.43341108700271342816e-1 * t40019 + 0.10975748638225852664e0 * t40021;
    (t40023,)
}
