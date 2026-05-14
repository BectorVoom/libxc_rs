//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 889/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk889<F: Float>(t1652: F, t27055: F, t333: F, t352: F, t41116: F, t46622: F, t46669: F, t46671: F, t46673: F, t46675: F, t46677: F, t46679: F, t46686: F, t46694: F, t5155: F, t5266: F, t838: F, t8936: F, t8940: F) -> (F,) {
    let t46701 = -0.5987120850931904282e-1 * t46669 - 0.17961362552795712846e0 * t46671 - 0.17961362552795712846e0 * t46673 + 0.8980681276397856423e-1 * t46675 + 0.35922725105591425692e0 * t46677 + 0.23948483403727617128e0 * t838 * t46679 - 0.35922725105591425692e0 * t27055 * t46622 * t333 + 0.11974241701863808564e0 * t46686 + 0.23948483403727617128e0 * t8940 * t8936 * t1652 - 0.47896966807455234256e0 * t41116 * t46622 * t352 + 0.23948483403727617128e0 * t5266 * t46694 * t352 + 0.47896966807455234256e0 * t5155 * t46694 * t333;
    (t46701,)
}
