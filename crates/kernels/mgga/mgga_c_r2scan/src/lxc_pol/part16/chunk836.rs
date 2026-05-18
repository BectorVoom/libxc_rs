//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 836/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk836<F: Float>(t6134: F, t8820: F, t360: F, t277: F, t3216: F, t495: F, t3016: F, t3055: F, t537: F, t2124: F, t2551: F, t2892: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8821 = t8820 * t6134;
    let t8822 = t360 * t8821;
    let t8825 = t277 * t3216;
    let t8826 = t8825 * t495;
    let t8827 = t360 * t8826;
    let t8832 = t277 * t3016;
    let t8833 = t8832 * t495;
    let t8834 = t360 * t8833;
    let t8837 = t537 * t3055;
    let t8839 = t2124 * t8837 * t2551;
    let t8842 = t537 * t2892;
    (t8821, t8822, t8825, t8826, t8827, t8832, t8833, t8834, t8837, t8839, t8842)
}
