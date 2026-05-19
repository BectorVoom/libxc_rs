//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1015/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1015<F: Float>(t1743: F, t2064: F, t797: F, t2367: F, t558: F, t1652: F, t27055: F, t333: F, t352: F, t41116: F, t46622: F, t46669: F, t46671: F, t46673: F, t46675: F, t46677: F, t46679: F, t5155: F, t5266: F, t838: F, t8936: F, t8940: F) -> (F, F, F) {
    let t46685 = t2064 * t1743;
    let t46686 = t797 * t46685;
    let t46694 = t2367 * t558;
    let t46701 = -F::cast_from(0.5987120850931904282e-1_f64) * t46669 - F::cast_from(0.17961362552795712846e0_f64) * t46671 - F::cast_from(0.17961362552795712846e0_f64) * t46673 + F::cast_from(0.8980681276397856423e-1_f64) * t46675 + F::cast_from(0.35922725105591425692e0_f64) * t46677 + F::cast_from(0.23948483403727617128e0_f64) * t838 * t46679 - F::cast_from(0.35922725105591425692e0_f64) * t27055 * t46622 * t333 + F::cast_from(0.11974241701863808564e0_f64) * t46686 + F::cast_from(0.23948483403727617128e0_f64) * t8940 * t8936 * t1652 - F::cast_from(0.47896966807455234256e0_f64) * t41116 * t46622 * t352 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t46694 * t352 + F::cast_from(0.47896966807455234256e0_f64) * t5155 * t46694 * t333;
    (t46685, t46694, t46701)
}
