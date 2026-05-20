//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3121/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3121<F: Float>(t11792: F, t4845: F, t15749: F, t3224: F, t11922: F, t16039: F, t3115: F, t11859: F, t15610: F, t1032: F, t1040: F, t15886: F) -> (F, F, F, F, F) {
    let t55152 = t11792 * t4845;
    let t55154 = t3224 * t15749;
    let t55171 = t3115 * t11922 * t16039;
    let t55182 = t11859 * t11922 * t15610;
    let t55195 = t15886 * t1032 * t1040;
    (t55152, t55154, t55171, t55182, t55195)
}
