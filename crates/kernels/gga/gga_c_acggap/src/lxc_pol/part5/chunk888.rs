//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 888/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk888<F: Float>(t1137: F, t4787: F, t3375: F, t4372: F, t1163: F, t1165: F, t4162: F, t4289: F, t157: F, t406: F, t864: F) -> (F, F, F, F) {
    let t15748 = t1137 * t4787;
    let t15750 = t3375 * t4372;
    let t15754 = t1163 * t1165 * t4289 * t4162;
    let t15758 = t864 * t406 * t157;
    (t15748, t15750, t15754, t15758)
}
