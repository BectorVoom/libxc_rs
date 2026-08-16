//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 901/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk901(t41666: f64, t41669: f64, t41674: f64, t2478: f64, t3541: f64, t6583: f64, t41676: f64, t41683: f64, t41686: f64, t41689: f64, t41691: f64, t41696: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46128 = 0.41708904943825497782e0_f64 * t41666;
    let t46129 = 0.11916829983950142223e0_f64 * t41669;
    let t46131 = 0.15337170381568299871e1_f64 * t41674;
    let t46138 = t6583 * t3541 * t2478;
    let t46167 = 0.59584149919750711116e-1_f64 * t41676;
    let t46168 = 0.15337170381568299871e1_f64 * t41683;
    let t46169 = 0.15337170381568299871e1_f64 * t41686;
    let t46170 = 0.34082600847929555269e0_f64 * t41689;
    let t46174 = 0.59584149919750711116e-1_f64 * t41691;
    let t46175 = 0.35750489951850426669e0_f64 * t41696;
    (t46128, t46129, t46131, t46138, t46167, t46168, t46169, t46170, t46174, t46175)
}
