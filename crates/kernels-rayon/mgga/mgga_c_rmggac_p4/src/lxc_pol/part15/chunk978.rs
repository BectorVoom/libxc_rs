//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 978/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk978(t1704: f64, t265: f64, t262: f64, t7648: f64, t1737: f64, t7653: f64, t36094: f64, t36096: f64, t46150: f64, t46152: f64, t46154: f64, t46156: f64, t46158: f64, t46160: f64, t46162: f64, t46165: f64, t46168: f64, t46170: f64, t46172: f64, t46178: f64) -> (f64, f64, f64, f64, f64) {
    let t46180 = t265 * t1704;
    let t46181 = t262 * t46180;
    let t46182 = t7648 * t46181;
    let t46184 = t265 * t1737;
    let t46185 = t262 * t46184;
    let t46186 = t7653 * t46185;
    let t46188 = -0.31752135234603223702e-2_f64 * t46150 + 0.9072038638458063915e-3_f64 * t46152 - 0.12700854093841289481e-2_f64 * t46154 - 0.12700854093841289481e-2_f64 * t46156 + 0.50803416375365157926e-2_f64 * t46158 - 0.7620512456304773689e-2_f64 * t46160 + 0.16934472125121719309e-2_f64 * t46162 + 0.45360193192290319575e-3_f64 * t46165 - 0.63504270469206447405e-3_f64 * t46168 - 0.63504270469206447408e-3_f64 * t46170 + 0.84672360625608596544e-3_f64 * t46172 + 0.33335697577410973224e-1_f64 * t36094 - 0.44447596769881297632e-1_f64 * t36096 + 0.12122071846331262991e-1_f64 * t46178 - 0.1209605151794408522e-2_f64 * t46182 - 0.22579296166828959078e-2_f64 * t46186;
    (t46180, t46181, t46184, t46185, t46188)
}
