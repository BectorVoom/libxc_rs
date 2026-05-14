//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 962/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk962<F: Float>(t10734: F, t571: F, t572: F, t22948: F, t37945: F, t254: F, t259: F, t277: F, t37449: F, t2116: F, t57: F, t6257: F, t505: F, t6159: F, t6162: F, t2096: F, t2105: F, t265: F, t6079: F) -> (F, F, F, F, F, F) {
    let t38031 = t571 * t572 * t10734;
    let t38033 = t38031 * t37945 * t22948;
    let t38054 = t254 * t259 * t37449 * t277;
    let t38055 = 0.19776387377308997907e1 * t38054;
    let t38068 = t6257 * t57 * t2116;
    let t38069 = 0.98171973930797904389e-1 * t38068;
    let t38130 = t6159 * t505 * t6162;
    let t38131 = 0.14457274399185490173e-4 * t38130;
    let t38143 = t254 * t6079 * t2096 * t265 * t2105;
    (t38031, t38033, t38055, t38069, t38131, t38143)
}
