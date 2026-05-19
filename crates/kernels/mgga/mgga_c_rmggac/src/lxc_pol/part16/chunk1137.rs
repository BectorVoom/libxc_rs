//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1137/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1137<F: Float>(t1704: F, t698: F, t10252: F, t118: F, t1587: F, t1614: F, t25820: F, t27101: F, t333: F, t352: F, t41521: F, t41522: F, t44337: F, t46748: F, t46750: F, t49210: F, t49560: F, t5148: F, t5155: F, t5259: F, t6444: F, t9523: F, t9540: F) -> F {
    let t49572 = t698 * t1704;
    let t49591 = F::cast_from(0.47896966807455234256e0_f64) * t5155 * t9540 * t1614 - F::cast_from(0.35922725105591425692e0_f64) * t25820 * t49572 * t333 + F::cast_from(0.11974241701863808564e0_f64) * t46748 - F::cast_from(0.23948483403727617128e0_f64) * t27101 * t49572 * t352 - F::cast_from(0.17961362552795712846e0_f64) * t46750 - F::cast_from(0.79828278012425390428e-1_f64) * t118 * t49210 + F::cast_from(0.23948483403727617128e0_f64) * t5259 * t9523 * t1587 + t41521 - t41522 + t44337 + F::cast_from(0.11974241701863808564e0_f64) * t6444 * t10252 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t49560 * t352;
    t49591
}
