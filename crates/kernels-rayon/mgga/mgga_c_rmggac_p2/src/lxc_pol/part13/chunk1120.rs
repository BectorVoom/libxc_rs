//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1120/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1120(t42151: f64, t42166: f64, t42177: f64, t42180: f64, t290: f64, t9595: f64, t1664: f64, t2231: f64, t1288: f64, t1356: f64, t1364: f64, t1540: f64, t2211: f64, t2262: f64, t2474: f64, t27177: f64, t289: f64, t30900: f64, t36976: f64, t37419: f64, t38140: f64, t42156: f64, t42159: f64, t42162: f64, t42170: f64, t42174: f64, t44252: f64, t72: f64, t739: f64) -> f64 {
    let t44385 = 0.47896966807455234256e0_f64 * t42151;
    let t44396 = 0.21819729323396273384e0_f64 * t42166;
    let t44399 = 0.39726959900411316772e-4_f64 * t42177;
    let t44400 = 0.39726959900411316772e-4_f64 * t42180;
    let t44405 = t290 * t9595;
    let t44410 = t1664 * t2231;
    let t44413 = -t44385 - 0.2993560425465952141e-1_f64 * t42156 + 0.43639458646792546768e0_f64 * t36976 - t38140 - 0.35922725105591425692e0_f64 * t42159 - 0.11974241701863808564e0_f64 * t42162 + 0.47896966807455234256e0_f64 * t1364 * t2211 * t27177 + 0.47896966807455234256e0_f64 * t1356 * t37419 * t30900 - t44396 - 0.1440846329149835838e-2_f64 * t42170 + 0.20496175532535769482e-3_f64 * t42174 - t44399 - t44400 - 0.11974241701863808564e0_f64 * t739 * t44252 - 0.39914139006212695214e-1_f64 * t1540 * t2262 - 0.4726e1_f64 * t289 * t44405 + t72 * t1288 * t2474 - 0.4726e1_f64 * t289 * t44410;
    t44413
}
