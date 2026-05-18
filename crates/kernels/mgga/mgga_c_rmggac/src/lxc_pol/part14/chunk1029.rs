//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1029/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1029<F: Float>(t25854: F, t40887: F, t4905: F, t8975: F, t2301: F, t5245: F, t2295: F, t30510: F, t40883: F, t5259: F, t333: F, t352: F, t36013: F, t36035: F, t41015: F, t41059: F, t41122: F, t4669: F, t5148: F, t5155: F, t5266: F) -> (F, F) {
    let t41458 = t25854 * t40887;
    let t41460 = t8975 * t4905;
    let t41463 = t5245 * t2301;
    let t41475 = t30510 * t2295;
    let t41477 = t5259 * t40883;
    let t41482 = F::new(0.23948483403727617128e0) * t5266 * t41015 * t333 - F::new(0.8980681276397856423e-1) * t41458 + F::new(0.71845450211182851384e0) * t25854 * t41460 - F::new(0.2993560425465952141e-1) * t41463 + F::new(0.11974241701863808564e0) * t36013 + t36035 - F::new(0.35922725105591425692e0) * t4669 * t41059 * t333 - F::new(0.23948483403727617128e0) * t5148 * t41059 * t352 + F::new(0.47896966807455234256e0) * t5155 * t41122 * t333 + F::new(0.2993560425465952141e-1) * t41475 - F::new(0.2993560425465952141e-1) * t41477 + F::new(0.23948483403727617128e0) * t5266 * t41122 * t352;
    (t41460, t41482)
}
