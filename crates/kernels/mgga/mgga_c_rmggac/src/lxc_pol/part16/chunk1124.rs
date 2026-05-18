//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1124/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1124<F: Float>(t25820: F, t25854: F, t25877: F, t27055: F, t27101: F, t40891: F, t40899: F, t40911: F, t40922: F, t44070: F, t44075: F, t46076: F, t46320: F, t48259: F, t48262: F, t48265: F, t48268: F, t48287: F) -> F {
    let t49311 = -F::new(0.8980681276397856423e-1) * t46076 + F::new(0.1454648621559751559e0) * t40891 - F::new(0.4363945864679254677e0) * t40899 + t44070 - F::new(0.43639458646792546768e0) * t40911 - t44075 + F::new(0.7273243107798757795e0) * t40922 - F::new(0.71845450211182851384e0) * t27055 * t48287 - F::new(0.47896966807455234256e0) * t27101 * t48268 + F::new(0.54549323308490683461e-1) * t46320 - F::new(0.71845450211182851384e0) * t25820 * t48259 + F::new(0.14369090042236570277e1) * t25877 * t48262 + F::new(0.71845450211182851384e0) * t25854 * t48265;
    t49311
}
