//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 955/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk955<F: Float>(t41676: F, t41683: F, t41686: F, t41689: F, t41691: F, t41696: F, t41698: F, t188: F, t44337: F, t38051: F, t544: F, t9287: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46167 = F::new(0.59584149919750711116e-1) * t41676;
    let t46168 = F::new(0.15337170381568299871e1) * t41683;
    let t46169 = F::new(0.15337170381568299871e1) * t41686;
    let t46170 = F::new(0.34082600847929555269e0) * t41689;
    let t46174 = F::new(0.59584149919750711116e-1) * t41691;
    let t46175 = F::new(0.35750489951850426669e0) * t41696;
    let t46176 = F::new(0.20449560508757733161e1) * t41698;
    let t46181 = t188 * t44337;
    let t46189 = t544 * t38051 * t9287;
    (t46167, t46168, t46169, t46170, t46174, t46175, t46176, t46181, t46189)
}
