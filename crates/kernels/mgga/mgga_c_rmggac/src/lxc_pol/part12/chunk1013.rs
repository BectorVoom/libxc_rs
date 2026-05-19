//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1013/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1013<F: Float>(t41035: F, t854: F, t3826: F, t39688: F, t3810: F, t39684: F, t39879: F, t40920: F, t3839: F, t39055: F, t39059: F, t41031: F) -> (F, F, F, F, F, F, F, F) {
    let t41233 = t854 * t41035;
    let t41234 = F::cast_from(0.21241846568096930142e-2_f64) * t41233;
    let t41235 = t3826 * t39688;
    let t41237 = t3810 * t39684;
    let t41239 = t3826 * t39879;
    let t41241 = t3810 * t40920;
    let t41242 = F::cast_from(0.14869292597667851099e-1_f64) * t41241;
    let t41243 = t3839 * t39055;
    let t41245 = t3826 * t39059;
    let t41247 = t854 * t41031;
    (t41234, t41235, t41237, t41239, t41242, t41243, t41245, t41247)
}
