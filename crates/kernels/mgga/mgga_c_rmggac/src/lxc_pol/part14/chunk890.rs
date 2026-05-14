//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 890/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk890<F: Float>(t2347: F, t30510: F, t36110: F, t41000: F, t36103: F, t41150: F, t41027: F, t793: F, t2350: F, t26531: F, t41035: F, t797: F, t41043: F, t851: F, t41166: F, t41168: F, t41171: F, t41172: F, t41174: F, t41177: F, t41179: F, t41181: F, t41183: F) -> (F,) {
    let t41185 = t30510 * t2347;
    let t41187 = t36110 * t41000;
    let t41189 = t36103 * t41150;
    let t41191 = t793 * t41027;
    let t41193 = t26531 * t2350;
    let t41195 = t797 * t41035;
    let t41197 = t851 * t41043;
    let t41199 = -0.23948483403727617128e0 * t41166 + 0.79656924630363488032e-2 * t41168 - t41171 + 0.39828462315181744016e-2 * t41172 - 0.55759847241254441622e-2 * t41174 - 0.27879923620627220812e-1 * t41177 - 0.19957069503106347607e-1 * t41179 - 0.99785347515531738034e-2 * t41181 + 0.14967802127329760705e-1 * t41183 - 0.99785347515531738034e-2 * t41185 + 0.10160683275073031585e-1 * t41187 - 0.63504270469206447404e-2 * t41189 - 0.97567895348519921633e-1 * t41191 + 0.2993560425465952141e-1 * t41193 - 0.79828278012425390426e-1 * t41195 - 0.33190385262651453347e-3 * t41197;
    (t41199,)
}
