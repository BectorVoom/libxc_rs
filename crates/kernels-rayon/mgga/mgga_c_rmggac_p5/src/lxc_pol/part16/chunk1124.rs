//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1124/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1124(t25820: f64, t25854: f64, t25877: f64, t27055: f64, t27101: f64, t40891: f64, t40899: f64, t40911: f64, t40922: f64, t44070: f64, t44075: f64, t46076: f64, t46320: f64, t48259: f64, t48262: f64, t48265: f64, t48268: f64, t48287: f64) -> f64 {
    let t49311 = -0.8980681276397856423e-1_f64 * t46076 + 0.1454648621559751559e0_f64 * t40891 - 0.4363945864679254677e0_f64 * t40899 + t44070 - 0.43639458646792546768e0_f64 * t40911 - t44075 + 0.7273243107798757795e0_f64 * t40922 - 0.71845450211182851384e0_f64 * t27055 * t48287 - 0.47896966807455234256e0_f64 * t27101 * t48268 + 0.54549323308490683461e-1_f64 * t46320 - 0.71845450211182851384e0_f64 * t25820 * t48259 + 0.14369090042236570277e1_f64 * t25877 * t48262 + 0.71845450211182851384e0_f64 * t25854 * t48265;
    t49311
}
