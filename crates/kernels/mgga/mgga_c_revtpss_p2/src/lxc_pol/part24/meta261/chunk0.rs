//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1031/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1031<F: Float>(t17352: F, t372: F, t11262: F, t1796: F, t1247: F, t1770: F, t3140: F, t3609: F, t1802: F, t474: F, t3089: F) -> (F, F, F, F, F, F, F) {
    let t17353 = t372 * t17352;
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17376 = t1770 * t3140;
    let t17377 = t17376 * t3609;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    (t17353, t17361, t17362, t17376, t17377, t17394, t17395)
}
