//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 985/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk985<F: Float>(t1743: F, t698: F, t1756: F, t118: F, t326: F, t333: F, t352: F, t44169: F, t46599: F, t46603: F, t46605: F, t46607: F, t46609: F, t48122: F, t48894: F, t48897: F, t49394: F, t5155: F, t5266: F, t8940: F) -> (F, F, F) {
    let t49407 = t698 * t1743;
    let t49411 = t698 * t1756;
    let t49424 = 0.47896966807455234256e0 * t5155 * t49394 * t333 + t44169 - 0.39914139006212695214e-1 * t118 * t48122 + 0.11974241701863808564e0 * t5266 * t49407 * t352 + 0.11974241701863808564e0 * t8940 * t49411 * t352 + 0.11974241701863808564e0 * t46599 + 0.95793933614910468512e0 * t46603 - 0.5987120850931904282e-1 * t46605 + 0.8980681276397856423e-1 * t46607 - 0.59871208509319042821e-1 * t326 * t48894 - 0.11974241701863808564e0 * t326 * t48897 + 0.5987120850931904282e-1 * t46609;
    (t49407, t49411, t49424)
}
