//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 976/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk976<F: Float>(t2227: F, t558: F, t1587: F, t698: F, t2447: F, t321: F, t9565: F, t25854: F, t27055: F, t305: F, t333: F, t352: F, t36045: F, t41475: F, t41477: F, t42634: F, t42640: F, t44011: F, t44187: F, t4669: F, t5148: F, t5259: F, t5266: F, t838: F, t866: F, t9523: F) -> (F, F, F, F, F) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44244 = t2447 * t321;
    let t44252 = t9565 * t321;
    let t44264 = 0.5987120850931904282e-1 * t41475 + 0.23948483403727617128e0 * t5266 * t44232 * t352 - 0.11974241701863808564e0 * t5148 * t9523 * t866 - 0.23948483403727617128e0 * t5148 * t44239 * t352 - 0.5987120850931904282e-1 * t41477 - 0.35922725105591425692e0 * t4669 * t44244 * t333 - 0.71845450211182851384e0 * t27055 * t42640 + 0.71845450211182851384e0 * t25854 * t42634 + 0.11974241701863808564e0 * t305 * t44252 + 0.23948483403727617128e0 * t838 * t44011 - 0.35922725105591425692e0 * t4669 * t44187 * t333 - 0.15965655602485078085e0 * t36045 + 0.23948483403727617128e0 * t5259 * t44239 * t321;
    (t44232, t44239, t44244, t44252, t44264)
}
