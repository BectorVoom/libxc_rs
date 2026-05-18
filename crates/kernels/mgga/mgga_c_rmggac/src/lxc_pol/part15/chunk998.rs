//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 998/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk998<F: Float>(t262: F, t46427: F, t7835: F, t46278: F, t7844: F, t46261: F, t7785: F, t352: F, t9872: F, t7788: F, t2350: F, t5144: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46428 = t262 * t46427;
    let t46429 = t7835 * t46428;
    let t46431 = t262 * t46278;
    let t46432 = t7844 * t46431;
    let t46434 = t262 * t46261;
    let t46435 = t7785 * t46434;
    let t46437 = t9872 * t352;
    let t46438 = t262 * t46437;
    let t46439 = t7788 * t46438;
    let t46441 = t2350 * t5144;
    (t46428, t46429, t46431, t46432, t46434, t46435, t46437, t46438, t46439, t46441)
}
