//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 955/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk955(t41676: f64, t41683: f64, t41686: f64, t41689: f64, t41691: f64, t41696: f64, t41698: f64, t188: f64, t44337: f64, t38051: f64, t544: f64, t9287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46167 = 0.59584149919750711116e-1_f64 * t41676;
    let t46168 = 0.15337170381568299871e1_f64 * t41683;
    let t46169 = 0.15337170381568299871e1_f64 * t41686;
    let t46170 = 0.34082600847929555269e0_f64 * t41689;
    let t46174 = 0.59584149919750711116e-1_f64 * t41691;
    let t46175 = 0.35750489951850426669e0_f64 * t41696;
    let t46176 = 0.20449560508757733161e1_f64 * t41698;
    let t46181 = t188 * t44337;
    let t46189 = t544 * t38051 * t9287;
    (t46167, t46168, t46169, t46170, t46174, t46175, t46176, t46181, t46189)
}
