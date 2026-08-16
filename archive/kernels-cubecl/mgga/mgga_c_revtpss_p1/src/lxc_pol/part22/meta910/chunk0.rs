//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3113/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113<F: Float>(t15731: F, t3169: F, t15816: F, t3168: F, t11710: F, t15591: F, t3091: F, t16060: F, t3241: F, t1011: F, t140: F, t16122: F) -> (F, F, F, F, F) {
    let t54733 = t3169 * t15731;
    let t54739 = t15816 * t3168;
    let t54785 = t3091 * t11710 * t15591;
    let t54792 = t3241 * t16060;
    let t54795 = t1011 * t140 * t16122;
    (t54733, t54739, t54785, t54792, t54795)
}
