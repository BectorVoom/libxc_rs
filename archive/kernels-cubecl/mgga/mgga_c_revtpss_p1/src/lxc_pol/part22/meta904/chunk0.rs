//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3101/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3101<F: Float>(t1011: F, t3252: F, t4574: F, t697: F, t1062: F, t15887: F, t11921: F, t15837: F, t247: F, t4837: F, t11267: F, t4878: F) -> (F, F, F, F) {
    let t54126 = t1011 * t697 * t3252 * t4574;
    let t54137 = t15887 * t1062;
    let t54142 = t4837 * t247 * t11921 * t15837;
    let t54144 = t4878 * t11267;
    (t54126, t54137, t54142, t54144)
}
