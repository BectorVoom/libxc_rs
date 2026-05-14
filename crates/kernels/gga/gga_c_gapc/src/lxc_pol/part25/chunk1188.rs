//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1188/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1188<F: Float>(t35095: F, t35097: F, t35108: F, t35121: F, t35124: F, t35127: F, t35132: F, t35135: F, t35137: F, t35141: F, t35143: F, t35146: F, t35152: F, t35155: F, t35157: F, t35160: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37193 = 0.27012148473991046866e-5 * t35095;
    let t37194 = 0.11594181388521408695e-4 * t35097;
    let t37200 = 0.20220636637604418766e-5 * t35108;
    let t37205 = 0.21135226489492151266e-6 * t35121;
    let t37206 = 0.19808908880926767702e-4 * t35124;
    let t37207 = 0.57920616843011475696e-5 * t35127;
    let t37208 = 0.50680539737635041234e-3 * t35132;
    let t37210 = 0.43284943850479925795e-3 * t35135;
    let t37211 = 0.3243554543208642639e-2 * t35137;
    let t37212 = 0.61551119569641057312e-8 * t35141;
    let t37213 = 0.27012148473991046866e-5 * t35143;
    let t37214 = 0.11372686522837130914e-5 * t35146;
    let t37216 = 0.337303223138432284e-8 * t35152;
    let t37217 = 0.55331893559454114829e-8 * t35155;
    let t37218 = 0.99044544404633838508e-5 * t35157;
    let t37219 = 0.33816362383187442026e-5 * t35160;
    (t37193, t37194, t37200, t37205, t37206, t37207, t37208, t37210, t37211, t37212, t37213, t37214, t37216, t37217, t37218, t37219)
}
