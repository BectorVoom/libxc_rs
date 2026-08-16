//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1227/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1227<F: Float>(t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t4430: F, t93034: F, t1565: F, t93066: F, t4349: F, t93072: F) -> (F, F, F, F, F) {
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t99002 = t93034 * t4430;
    let t99009 = t93066 * t1565;
    let t99013 = t93072 * t4349;
    (t98976, t98979, t99002, t99009, t99013)
}
