//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1042/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1042<F: Float>(t27067: F, t3610: F, t37600: F, t39429: F, t39438: F, t39440: F, t39444: F, t39446: F, t39459: F, t39482: F, t41395: F, t41397: F, t43057: F, t29274: F, t3332: F, t7614: F) -> (F, F) {
    let t43061 = t27067 * t3610;
    let t43064 = 0.54878743191129263322e-1 * t43057 + 0.31147743054556651236e-1 * t39429 + t39438 - 0.95219938395347901944e-2 * t39440 - t39444 + t39446 - t39459 - t41395 - t41397 + 0.43663693315433241792e-2 * t43061 - t37600 + 0.31147743054556651236e-1 * t39482;
    let t43072 = t7614 * t3332 * t29274;
    (t43064, t43072)
}
