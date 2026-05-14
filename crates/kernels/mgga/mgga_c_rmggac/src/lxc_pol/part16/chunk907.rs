//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 907/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk907<F: Float>(t41738: F, t46542: F, t4044: F, t6400: F, t645: F, t4601: F, t9739: F, t2060: F, t30283: F, t903: F, t30360: F, t46502: F, t7204: F, t46358: F, t8447: F, t8577: F) -> (F, F, F, F, F, F, F, F) {
    let t47729 = t41738 * t46542;
    let t47735 = t4044 * t645 * t6400;
    let t47737 = t4601 * t9739;
    let t47740 = t903 * t2060 * t30283;
    let t47743 = t903 * t2060 * t30360;
    let t47745 = t7204 * t46502;
    let t47747 = t7204 * t46358;
    let t47757 = t8577 * t8447;
    (t47729, t47735, t47737, t47740, t47743, t47745, t47747, t47757)
}
