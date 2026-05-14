//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1094/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1094<F: Float>(t12435: F, t3308: F, t3429: F, t1102: F, t3314: F, t37387: F, t39261: F, t42431: F, t42435: F, t42437: F, t42441: F, t42443: F, t42447: F, t42450: F, t42452: F, t42457: F, t42460: F, t43826: F) -> (F,) {
    let t43829 = t3429 * t3308 * t12435;
    let t43832 = t1102 * t3314 * t12435;
    let t43834 = -t42431 + t42435 - 0.36021158228745895953e-3 * t43826 - 0.15243824895787514157e-3 * t43829 - 0.40650199722100037752e-3 * t43832 + t42437 - t42441 - t42443 - t37387 + t42447 - t42450 - t39261 + t42452 + t42457 - t42460;
    (t43834,)
}
