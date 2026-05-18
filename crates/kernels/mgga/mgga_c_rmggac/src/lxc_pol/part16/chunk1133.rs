//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1133/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1133<F: Float>(t1737: F, t698: F, t25877: F, t321: F, t352: F, t44187: F, t44194: F, t46642: F, t46646: F, t46648: F, t46650: F, t46652: F, t46656: F, t46658: F, t46660: F, t49445: F, t5148: F, t5266: F, t570: F) -> (F, F) {
    let t49469 = t698 * t1737;
    let t49475 = F::new(0.23948483403727617128e0) * t5266 * t44194 * t570 - F::new(0.11974241701863808564e0) * t5148 * t49445 * t352 + F::new(0.10909864661698136692e0) * t46642 + F::new(0.36366215538993788973e-1) * t46646 + F::new(0.36366215538993788973e0) * t46648 - F::new(0.20455996240684006298e-1) * t46650 + F::new(0.2727466165424534173e-1) * t46652 + F::new(0.68186654135613354325e-2) * t46656 - F::new(0.23948483403727617128e0) * t5148 * t44187 * t570 + F::new(0.71845450211182851384e0) * t25877 * t49469 * t321 + F::new(0.71845450211182851384e0) * t46658 - F::new(0.17961362552795712846e1) * t46660;
    (t49469, t49475)
}
