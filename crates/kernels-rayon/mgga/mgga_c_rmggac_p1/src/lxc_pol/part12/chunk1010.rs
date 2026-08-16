//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1010/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1010(t41035: f64, t797: f64, t41043: f64, t851: f64, t41166: f64, t41168: f64, t41171: f64, t41172: f64, t41174: f64, t41177: f64, t41179: f64, t41181: f64, t41183: f64, t41185: f64, t41187: f64, t41189: f64, t41191: f64, t41193: f64) -> f64 {
    let t41195 = t797 * t41035;
    let t41197 = t851 * t41043;
    let t41199 = -0.23948483403727617128e0_f64 * t41166 + 0.79656924630363488032e-2_f64 * t41168 - t41171 + 0.39828462315181744016e-2_f64 * t41172 - 0.55759847241254441622e-2_f64 * t41174 - 0.27879923620627220812e-1_f64 * t41177 - 0.19957069503106347607e-1_f64 * t41179 - 0.99785347515531738034e-2_f64 * t41181 + 0.14967802127329760705e-1_f64 * t41183 - 0.99785347515531738034e-2_f64 * t41185 + 0.10160683275073031585e-1_f64 * t41187 - 0.63504270469206447404e-2_f64 * t41189 - 0.97567895348519921633e-1_f64 * t41191 + 0.2993560425465952141e-1_f64 * t41193 - 0.79828278012425390426e-1_f64 * t41195 - 0.33190385262651453347e-3_f64 * t41197;
    t41199
}
