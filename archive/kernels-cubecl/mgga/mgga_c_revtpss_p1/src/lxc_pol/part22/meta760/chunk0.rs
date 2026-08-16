//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2840/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2840<F: Float>(t11670: F, t11772: F, t3114: F, t11773: F, t11926: F, t11858: F, t15688: F, t16102: F, t3155: F, t12077: F, t15905: F, t994: F) -> (F, F, F, F, F, F) {
    let t43065 = t11670 * t11772;
    let t43066 = t3114 * t43065;
    let t43069 = t11926 * t11773;
    let t43082 = t11858 * t15688;
    let t43085 = t3155 * t16102;
    let t43105 = t994 * t12077 * t15905;
    (t43065, t43066, t43069, t43082, t43085, t43105)
}
