//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 986/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk986<F: Float>(t3810: F, t46258: F, t3851: F, t45418: F, t3826: F, t46261: F, t15093: F, t8704: F, t2074: F, t46068: F, t321: F, t9872: F) -> (F, F, F, F, F, F, F) {
    let t46266 = t3810 * t46258;
    let t46268 = t3851 * t45418;
    let t46270 = t3826 * t46261;
    let t46272 = t3826 * t45418;
    let t46274 = t15093 * t8704;
    let t46276 = t46068 * t2074;
    let t46278 = t9872 * t321;
    (t46266, t46268, t46270, t46272, t46274, t46276, t46278)
}
