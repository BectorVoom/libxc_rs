//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1427/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1427<F: Float>(t3173: F, t4879: F, t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t3252: F) -> (F, F, F, F, F) {
    let t15942 = F::cast_from(0.28582678745379824648e-3_f64) * t4879 * t3173;
    let t15957 = t4866 * t73;
    let t15984 = t11710 * t4782;
    let t15986 = F::cast_from(0.19055119163586549765e-3_f64) * t3091 * t15984;
    let t15987 = t140 * t1014;
    let t15988 = t15987 * t4579;
    let t15990 = t1011 * t15988 / F::cast_from(216.0_f64);
    let t15993 = t140 * t3252;
    (t15942, t15957, t15986, t15990, t15993)
}
