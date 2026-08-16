//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1120/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1120<F: Float>(t42151: F, t42166: F, t42177: F, t42180: F, t290: F, t9595: F, t1664: F, t2231: F, t1288: F, t1356: F, t1364: F, t1540: F, t2211: F, t2262: F, t2474: F, t27177: F, t289: F, t30900: F, t36976: F, t37419: F, t38140: F, t42156: F, t42159: F, t42162: F, t42170: F, t42174: F, t44252: F, t72: F, t739: F) -> F {
    let t44385 = F::cast_from(0.47896966807455234256e0_f64) * t42151;
    let t44396 = F::cast_from(0.21819729323396273384e0_f64) * t42166;
    let t44399 = F::cast_from(0.39726959900411316772e-4_f64) * t42177;
    let t44400 = F::cast_from(0.39726959900411316772e-4_f64) * t42180;
    let t44405 = t290 * t9595;
    let t44410 = t1664 * t2231;
    let t44413 = -t44385 - F::cast_from(0.2993560425465952141e-1_f64) * t42156 + F::cast_from(0.43639458646792546768e0_f64) * t36976 - t38140 - F::cast_from(0.35922725105591425692e0_f64) * t42159 - F::cast_from(0.11974241701863808564e0_f64) * t42162 + F::cast_from(0.47896966807455234256e0_f64) * t1364 * t2211 * t27177 + F::cast_from(0.47896966807455234256e0_f64) * t1356 * t37419 * t30900 - t44396 - F::cast_from(0.1440846329149835838e-2_f64) * t42170 + F::cast_from(0.20496175532535769482e-3_f64) * t42174 - t44399 - t44400 - F::cast_from(0.11974241701863808564e0_f64) * t739 * t44252 - F::cast_from(0.39914139006212695214e-1_f64) * t1540 * t2262 - F::cast_from(0.4726e1_f64) * t289 * t44405 + t72 * t1288 * t2474 - F::cast_from(0.4726e1_f64) * t289 * t44410;
    t44413
}
