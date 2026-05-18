//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 968/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk968<F: Float>(t41227: F, t8746: F, t41055: F, t851: F, t41035: F, t854: F, t3826: F, t39688: F, t3810: F, t39684: F, t39879: F, t40920: F) -> (F, F, F, F, F, F, F) {
    let t41228 = t8746 * t41227;
    let t41230 = t851 * t41055;
    let t41233 = t854 * t41035;
    let t41235 = t3826 * t39688;
    let t41237 = t3810 * t39684;
    let t41239 = t3826 * t39879;
    let t41241 = t3810 * t40920;
    (t41228, t41230, t41233, t41235, t41237, t41239, t41241)
}
