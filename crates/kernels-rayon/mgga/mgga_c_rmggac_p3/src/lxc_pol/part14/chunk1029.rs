//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1029/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1029(t25854: f64, t40887: f64, t4905: f64, t8975: f64, t2301: f64, t5245: f64, t2295: f64, t30510: f64, t40883: f64, t5259: f64, t333: f64, t352: f64, t36013: f64, t36035: f64, t41015: f64, t41059: f64, t41122: f64, t4669: f64, t5148: f64, t5155: f64, t5266: f64) -> (f64, f64) {
    let t41458 = t25854 * t40887;
    let t41460 = t8975 * t4905;
    let t41463 = t5245 * t2301;
    let t41475 = t30510 * t2295;
    let t41477 = t5259 * t40883;
    let t41482 = 0.23948483403727617128e0_f64 * t5266 * t41015 * t333 - 0.8980681276397856423e-1_f64 * t41458 + 0.71845450211182851384e0_f64 * t25854 * t41460 - 0.2993560425465952141e-1_f64 * t41463 + 0.11974241701863808564e0_f64 * t36013 + t36035 - 0.35922725105591425692e0_f64 * t4669 * t41059 * t333 - 0.23948483403727617128e0_f64 * t5148 * t41059 * t352 + 0.47896966807455234256e0_f64 * t5155 * t41122 * t333 + 0.2993560425465952141e-1_f64 * t41475 - 0.2993560425465952141e-1_f64 * t41477 + 0.23948483403727617128e0_f64 * t5266 * t41122 * t352;
    (t41460, t41482)
}
