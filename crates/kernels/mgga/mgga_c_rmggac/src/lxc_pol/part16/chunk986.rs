//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 986/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk986<F: Float>(t1756: F, t8264: F, t1734: F, t698: F, t118: F, t1652: F, t305: F, t321: F, t333: F, t338: F, t37584: F, t46612: F, t46614: F, t46634: F, t4669: F, t48489: F, t48539: F, t48591: F, t48638: F, t49154: F, t5259: F, t5266: F, t9540: F) -> (F, F, F) {
    let t49432 = t8264 * t1756;
    let t49445 = t698 * t1734;
    let t49452 = 0.5987120850931904282e-1 * t46612 - 0.47896966807455234256e0 * t46614 + 0.11974241701863808564e0 * t118 * t48539 + t37584 + 0.19957069503106347607e-1 * t118 * t338 * t49154 - 0.39914139006212695214e-1 * t118 * t49432 + 0.31931311204970156171e0 * t46634 + 0.23948483403727617128e0 * t5266 * t9540 * t1652 + 0.59871208509319042821e-1 * t305 * t48591 + 0.59871208509319042821e-1 * t305 * t48489 + 0.11974241701863808564e0 * t305 * t48638 + 0.11974241701863808564e0 * t5259 * t49445 * t321 - 0.17961362552795712846e0 * t4669 * t49445 * t333;
    (t49432, t49445, t49452)
}
