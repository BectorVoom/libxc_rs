//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 974/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk974<F: Float>(t10734: F, t571: F, t572: F, t22948: F, t37945: F, t10868: F, t6165: F, t6166: F, t254: F, t259: F, t277: F, t37449: F, t2080: F, t3316: F, t6535: F, t6536: F) -> (F, F, F, F, F, F) {
    let t38031 = t571 * t572 * t10734;
    let t38033 = t38031 * t37945 * t22948;
    let t38036 = t6165 * t10868 * t6166;
    let t38054 = t254 * t259 * t37449 * t277;
    let t38055 = 0.19776387377308997907e1 * t38054;
    let t38056 = t2080 * t3316;
    let t38062 = t6535 * t10868 * t6536;
    (t38031, t38033, t38036, t38055, t38056, t38062)
}
