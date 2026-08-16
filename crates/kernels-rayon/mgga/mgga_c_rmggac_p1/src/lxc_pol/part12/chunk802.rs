//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 802/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk802(t8400: f64, t8418: f64, t8423: f64, t8438: f64, t8444: f64, t8448: f64, t8452: f64, t8460: f64, t34544: f64, t34545: f64, t34548: f64, t7303: f64, t7307: f64, t9282: f64) -> (f64, f64, f64, f64) {
    let t38211 = 0.23948483403727617128e0_f64 * t8400;
    let t38212 = 0.17025839957319135759e-4_f64 * t8418;
    let t38213 = 0.85129199786595678796e-5_f64 * t8423;
    let t38217 = 0.85129199786595678796e-5_f64 * t8438;
    let t38218 = 0.85129199786595678796e-5_f64 * t8444;
    let t38219 = 0.85129199786595678796e-5_f64 * t8448;
    let t38220 = 0.85129199786595678796e-5_f64 * t8452;
    let t38221 = 0.39914139006212695214e-1_f64 * t8460;
    let t38224 = t38217 - t38218 - t38219 - t38220 + t9282 - t38221 + t34544 - t34545 - 0.60975299583150056628e-3_f64 * t7303 - 0.60975299583150056628e-3_f64 * t7307 + t34548;
    (t38211, t38212, t38213, t38224)
}
