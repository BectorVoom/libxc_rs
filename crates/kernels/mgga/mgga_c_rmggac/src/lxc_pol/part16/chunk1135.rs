//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1135/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1135<F: Float>(t43970: F, t570: F, t1704: F, t2228: F, t118: F, t27094: F, t27176: F, t321: F, t333: F, t352: F, t41500: F, t44232: F, t44239: F, t4669: F, t46702: F, t46707: F, t49407: F, t49411: F, t49469: F, t5155: F, t5266: F, t558: F, t793: F) -> (F, F, F) {
    let t49507 = t43970 * t570;
    let t49510 = t2228 * t1704;
    let t49533 = F::new(0.11974241701863808564e0) * t5266 * t49411 * t333 - F::new(0.79828278012425390428e-1) * t118 * t49507 + F::new(0.11974241701863808564e0) * t793 * t49510 - F::new(0.17961362552795712846e0) * t4669 * t49407 * t321 + F::new(0.23948483403727617128e0) * t5155 * t49407 * t333 + F::new(0.5987120850931904282e-1) * t46702 - F::new(0.15965655602485078085e0) * t46707 - t41500 - F::new(0.11974241701863808564e1) * t27094 * t49469 * t333 - F::new(0.47896966807455234256e0) * t27176 * t49469 * t352 + F::new(0.23948483403727617128e0) * t5266 * t44232 * t570 - F::new(0.35922725105591425692e0) * t4669 * t44239 * t558;
    (t49507, t49510, t49533)
}
