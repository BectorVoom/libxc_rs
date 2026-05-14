//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 945/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk945<F: Float>(t41170: F, t41191: F, t41195: F, t41166: F, t41168: F, t41172: F, t41174: F, t41177: F, t41179: F, t41181: F, t41183: F, t41185: F, t41187: F, t41189: F, t41193: F, t41197: F) -> (F,) {
    let t43518 = 0.21241846568096930142e-1 * t41170;
    let t43528 = 0.19513579069703984327e0 * t41191;
    let t43530 = 0.15965655602485078085e0 * t41195;
    let t43532 = -0.47896966807455234256e0 * t41166 + 0.15931384926072697607e-1 * t41168 - t43518 + 0.79656924630363488034e-2 * t41172 - 0.11151969448250888325e-1 * t41174 - 0.55759847241254441624e-1 * t41177 - 0.39914139006212695214e-1 * t41179 - 0.19957069503106347607e-1 * t41181 + 0.2993560425465952141e-1 * t41183 - 0.19957069503106347607e-1 * t41185 + 0.2032136655014606317e-1 * t41187 - 0.12700854093841289481e-1 * t41189 - t43528 + 0.5987120850931904282e-1 * t41193 - t43530 - 0.66380770525302906695e-3 * t41197;
    (t43532,)
}
