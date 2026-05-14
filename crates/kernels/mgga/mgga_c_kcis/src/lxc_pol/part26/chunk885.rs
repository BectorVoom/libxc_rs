//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 885/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk885<F: Float>(t22169: F, t5457: F, t1098: F, t7242: F, t3814: F, t531: F, t21641: F, t16373: F, t21625: F, t16359: F, t1319: F, t16582: F, t22114: F, t3255: F, t7222: F, t3780: F) -> (F, F, F, F, F, F, F, F) {
    let t22170 = t5457 * t22169;
    let t22175 = t1098 * t7242;
    let t22177 = t3814 * t531;
    let t22178 = t22177 * t21641;
    let t22181 = t16373 * t21625;
    let t22184 = t16359 * t21625;
    let t22188 = t16582 * t22114 * t1319;
    let t22191 = t3255 * t7222;
    let t22193 = t3780 * t531;
    (t22170, t22175, t22178, t22181, t22184, t22188, t22191, t22193)
}
