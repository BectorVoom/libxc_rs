//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1076/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1076<F: Float>(t2111: F, t2164: F, t22766: F, t20450: F, t2215: F, t10734: F, t571: F, t572: F, t22948: F, t37945: F, t254: F, t259: F, t277: F, t37449: F) -> (F, F, F, F, F) {
    let t38001 = t2111 * t22766 * t2164;
    let t38003 = t20450 * t2215;
    let t38031 = t571 * t572 * t10734;
    let t38033 = t38031 * t37945 * t22948;
    let t38054 = t254 * t259 * t37449 * t277;
    (t38001, t38003, t38031, t38033, t38054)
}
