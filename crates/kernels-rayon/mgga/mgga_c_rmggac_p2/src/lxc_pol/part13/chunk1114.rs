//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1114/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1114(t2227: f64, t558: f64, t1587: f64, t698: f64, t2447: f64, t321: f64, t9565: f64, t25854: f64, t27055: f64, t305: f64, t333: f64, t352: f64, t36045: f64, t41475: f64, t41477: f64, t42634: f64, t42640: f64, t44011: f64, t44187: f64, t4669: f64, t5148: f64, t5259: f64, t5266: f64, t838: f64, t866: f64, t9523: f64) -> (f64, f64, f64, f64, f64) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44244 = t2447 * t321;
    let t44252 = t9565 * t321;
    let t44264 = 0.5987120850931904282e-1_f64 * t41475 + 0.23948483403727617128e0_f64 * t5266 * t44232 * t352 - 0.11974241701863808564e0_f64 * t5148 * t9523 * t866 - 0.23948483403727617128e0_f64 * t5148 * t44239 * t352 - 0.5987120850931904282e-1_f64 * t41477 - 0.35922725105591425692e0_f64 * t4669 * t44244 * t333 - 0.71845450211182851384e0_f64 * t27055 * t42640 + 0.71845450211182851384e0_f64 * t25854 * t42634 + 0.11974241701863808564e0_f64 * t305 * t44252 + 0.23948483403727617128e0_f64 * t838 * t44011 - 0.35922725105591425692e0_f64 * t4669 * t44187 * t333 - 0.15965655602485078085e0_f64 * t36045 + 0.23948483403727617128e0_f64 * t5259 * t44239 * t321;
    (t44232, t44239, t44244, t44252, t44264)
}
