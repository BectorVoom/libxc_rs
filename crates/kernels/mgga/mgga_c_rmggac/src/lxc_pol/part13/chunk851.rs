//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 851/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk851<F: Float>(t3810: F, t39684: F, t3826: F, t39879: F, t40920: F, t3839: F, t39055: F, t39059: F, t41031: F, t854: F, t41047: F, t797: F, t25529: F, t36: F, t5169: F, t41027: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t41237 = t3810 * t39684;
    let t41239 = t3826 * t39879;
    let t41241 = t3810 * t40920;
    let t41243 = t3839 * t39055;
    let t41245 = t3826 * t39059;
    let t41247 = t854 * t41031;
    let t41255 = t854 * t41047;
    let t41257 = t797 * t41031;
    let t41262 = t25529 * t36;
    let t41263 = t41262 * t5169;
    let t41265 = t851 * t41027;
    (t41237, t41239, t41241, t41243, t41245, t41247, t41255, t41257, t41263, t41265)
}
