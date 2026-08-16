//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1079/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1079(t41170: f64, t41191: f64, t41195: f64, t41166: f64, t41168: f64, t41172: f64, t41174: f64, t41177: f64, t41179: f64, t41181: f64, t41183: f64, t41185: f64, t41187: f64, t41189: f64, t41193: f64, t41197: f64) -> f64 {
    let t43518 = 0.21241846568096930142e-1_f64 * t41170;
    let t43528 = 0.19513579069703984327e0_f64 * t41191;
    let t43530 = 0.15965655602485078085e0_f64 * t41195;
    let t43532 = -0.47896966807455234256e0_f64 * t41166 + 0.15931384926072697607e-1_f64 * t41168 - t43518 + 0.79656924630363488034e-2_f64 * t41172 - 0.11151969448250888325e-1_f64 * t41174 - 0.55759847241254441624e-1_f64 * t41177 - 0.39914139006212695214e-1_f64 * t41179 - 0.19957069503106347607e-1_f64 * t41181 + 0.2993560425465952141e-1_f64 * t41183 - 0.19957069503106347607e-1_f64 * t41185 + 0.2032136655014606317e-1_f64 * t41187 - 0.12700854093841289481e-1_f64 * t41189 - t43528 + 0.5987120850931904282e-1_f64 * t41193 - t43530 - 0.66380770525302906695e-3_f64 * t41197;
    t43532
}
