//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3102/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3102<F: Float>(t11263: F, t4879: F, t11773: F, t3278: F, t11875: F, t11922: F, t15898: F, t15728: F, t15827: F, t11672: F, t15984: F, t16052: F, t16055: F) -> (F, F, F, F, F, F) {
    let t54147 = t4879 * t11263;
    let t54166 = t3278 * t11773;
    let t54187 = t11875 * t11922 * t15898;
    let t54198 = t15728 * t15827;
    let t54222 = t11672 * t15984;
    let t54259 = t16052 * t16055;
    (t54147, t54166, t54187, t54198, t54222, t54259)
}
