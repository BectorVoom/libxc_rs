//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 981/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk981<F: Float>(t25854: F, t27048: F, t326: F, t40806: F, t44029: F, t44035: F, t46056: F, t46059: F, t46062: F, t46064: F, t46066: F, t46069: F, t48271: F, t48278: F, t48976: F, t25820: F, t25877: F, t27055: F, t27101: F, t40891: F, t40899: F, t40911: F, t40922: F, t44070: F, t44075: F, t46076: F, t46320: F, t48259: F, t48262: F, t48265: F, t48268: F, t48287: F) -> (F, F) {
    let t49294 = -0.59871208509319042821e-1 * t326 * t48976 - t44029 - 0.47896966807455234255e0 * t40806 - 0.95793933614910468512e0 * t46056 - 0.15965655602485078085e0 * t46059 + t44035 + 0.71845450211182851384e0 * t25854 * t48271 + 0.71845450211182851384e0 * t27048 * t48278 - 0.40911992481368012596e-1 * t46062 - 0.5987120850931904282e-1 * t46064 + 0.11974241701863808564e0 * t46066 - 0.17961362552795712846e0 * t46069;
    let t49311 = -0.8980681276397856423e-1 * t46076 + 0.1454648621559751559e0 * t40891 - 0.4363945864679254677e0 * t40899 + t44070 - 0.43639458646792546768e0 * t40911 - t44075 + 0.7273243107798757795e0 * t40922 - 0.71845450211182851384e0 * t27055 * t48287 - 0.47896966807455234256e0 * t27101 * t48268 + 0.54549323308490683461e-1 * t46320 - 0.71845450211182851384e0 * t25820 * t48259 + 0.14369090042236570277e1 * t25877 * t48262 + 0.71845450211182851384e0 * t25854 * t48265;
    (t49294, t49311)
}
