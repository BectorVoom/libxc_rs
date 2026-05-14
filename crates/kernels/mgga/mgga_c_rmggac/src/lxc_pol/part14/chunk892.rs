//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 892/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk892<F: Float>(t41035: F, t854: F, t3826: F, t39688: F, t3810: F, t39684: F, t39879: F, t40920: F, t3839: F, t39055: F, t39059: F, t41031: F, t41047: F, t797: F, t36094: F, t36096: F, t36099: F, t36101: F, t36115: F, t36117: F) -> (F,) {
    let t41233 = t854 * t41035;
    let t41234 = 0.21241846568096930142e-2 * t41233;
    let t41235 = t3826 * t39688;
    let t41237 = t3810 * t39684;
    let t41239 = t3826 * t39879;
    let t41241 = t3810 * t40920;
    let t41242 = 0.14869292597667851099e-1 * t41241;
    let t41243 = t3839 * t39055;
    let t41245 = t3826 * t39059;
    let t41247 = t854 * t41031;
    let t41255 = t854 * t41047;
    let t41257 = t797 * t41031;
    let t41259 = -t41234 + 0.19914231157590872008e-2 * t41235 - 0.27879923620627220811e-2 * t41237 + 0.19914231157590872008e-2 * t41239 + t41242 - 0.26552308210121162678e-2 * t41243 + 0.39828462315181744016e-2 * t41245 + 0.38943385374844371927e-2 * t41247 + 0.66671395154821946449e-1 * t36094 - 0.88895193539762595266e-1 * t36096 + 0.28224120208536198847e-3 * t36099 - 0.90915538847484472431e-2 * t36101 + 0.33868944250243438616e-2 * t36115 + 0.72732431077987577945e-1 * t36117 + 0.39828462315181744016e-3 * t41255 + 0.14635184302277988245e0 * t41257;
    (t41259,)
}
