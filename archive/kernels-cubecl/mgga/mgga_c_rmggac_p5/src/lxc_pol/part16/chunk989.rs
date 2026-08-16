//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 989/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk989<F: Float>(t6444: F, t9765: F, t5840: F, t645: F, t793: F, t46453: F, t4669: F, t44736: F, t5259: F, t46181: F, t7844: F, t1763: F, t262: F, t265: F, t7835: F) -> (F, F, F, F, F, F, F) {
    let t46609 = t6444 * t9765;
    let t46611 = t645 * t5840;
    let t46612 = t793 * t46611;
    let t46614 = t4669 * t46453;
    let t46634 = t5259 * t44736;
    let t46642 = t7844 * t46181;
    let t46646 = t7835 * t262 * t265 * t1763;
    (t46609, t46611, t46612, t46614, t46634, t46642, t46646)
}
