//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 635/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk635<F: Float>(t1776: F, t220: F, t7246: F, t2465: F, t25: F, t1773: F, t2464: F, t695: F, t1060: F, t5015: F, t1310: F, t657: F) -> (F, F, F, F, F, F, F) {
    let t7247 = t1776 * t220;
    let t7248 = t7246 * t7247;
    let t7253 = t25 * t2465;
    let t7254 = t1773 * t7253;
    let t7256 = t2464 * t695;
    let t7257 = t7256 * t1060;
    let t7258 = t5015 * t7257;
    let t7261 = t1310 * t657;
    (t7247, t7248, t7253, t7254, t7257, t7258, t7261)
}
