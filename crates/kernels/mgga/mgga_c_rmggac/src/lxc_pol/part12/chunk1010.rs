//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1010/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1010<F: Float>(t41035: F, t797: F, t41043: F, t851: F, t41166: F, t41168: F, t41171: F, t41172: F, t41174: F, t41177: F, t41179: F, t41181: F, t41183: F, t41185: F, t41187: F, t41189: F, t41191: F, t41193: F) -> F {
    let t41195 = t797 * t41035;
    let t41197 = t851 * t41043;
    let t41199 = -F::cast_from(0.23948483403727617128e0_f64) * t41166 + F::cast_from(0.79656924630363488032e-2_f64) * t41168 - t41171 + F::cast_from(0.39828462315181744016e-2_f64) * t41172 - F::cast_from(0.55759847241254441622e-2_f64) * t41174 - F::cast_from(0.27879923620627220812e-1_f64) * t41177 - F::cast_from(0.19957069503106347607e-1_f64) * t41179 - F::cast_from(0.99785347515531738034e-2_f64) * t41181 + F::cast_from(0.14967802127329760705e-1_f64) * t41183 - F::cast_from(0.99785347515531738034e-2_f64) * t41185 + F::cast_from(0.10160683275073031585e-1_f64) * t41187 - F::cast_from(0.63504270469206447404e-2_f64) * t41189 - F::cast_from(0.97567895348519921633e-1_f64) * t41191 + F::cast_from(0.2993560425465952141e-1_f64) * t41193 - F::cast_from(0.79828278012425390426e-1_f64) * t41195 - F::cast_from(0.33190385262651453347e-3_f64) * t41197;
    t41199
}
