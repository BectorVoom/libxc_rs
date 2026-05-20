//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3123/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123<F: Float>(t11988: F, t4834: F, t15731: F, t3124: F, t11933: F, t15794: F, t3115: F, t42793: F, t4911: F, t11951: F, t4858: F, t11922: F, t15906: F, t15909: F) -> (F, F, F, F, F, F) {
    let t55272 = t4834 * t11988;
    let t55279 = t3124 * t15731;
    let t55290 = t11933 * t15794;
    let t55293 = t3115 * t42793 * t4911;
    let t55320 = t4858 * t11951;
    let t55325 = t15906 * t11922 * t15909;
    (t55272, t55279, t55290, t55293, t55320, t55325)
}
