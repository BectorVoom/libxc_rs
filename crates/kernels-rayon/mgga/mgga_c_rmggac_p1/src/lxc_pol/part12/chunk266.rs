//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 266/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk266(t1036: f64, t1044: f64, t1047: f64, t1050: f64, t1087: f64, t1094: f64, t1104: f64, t1112: f64, t1133: f64, t1136: f64, t1140: f64, t1142: f64, t1143: f64, t1144: f64, t1243: f64, t196: f64, t500: f64) -> f64 {
    let t1247 = t1036 - t1044 - t1047 - t1050 + t1133 - t1094 + t1104 + t1112 - t1087 - t1136 + t1140 + t1142 + 0.186546e0_f64 * t1143 * t1144 + 0.31091e-1_f64 * t196 * t1243 * t500;
    t1247
}
