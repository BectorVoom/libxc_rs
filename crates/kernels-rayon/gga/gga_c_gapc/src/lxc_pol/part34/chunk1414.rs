//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1414/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1414(t35121: f64, t35124: f64, t35127: f64, t35132: f64, t35135: f64, t35137: f64, t35141: f64, t35143: f64, t35146: f64, t35152: f64, t35155: f64, t35157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37205 = 0.21135226489492151266e-6_f64 * t35121;
    let t37206 = 0.19808908880926767702e-4_f64 * t35124;
    let t37207 = 0.57920616843011475696e-5_f64 * t35127;
    let t37208 = 0.50680539737635041234e-3_f64 * t35132;
    let t37210 = 0.43284943850479925795e-3_f64 * t35135;
    let t37211 = 0.3243554543208642639e-2_f64 * t35137;
    let t37212 = 0.61551119569641057312e-8_f64 * t35141;
    let t37213 = 0.27012148473991046866e-5_f64 * t35143;
    let t37214 = 0.11372686522837130914e-5_f64 * t35146;
    let t37216 = 0.337303223138432284e-8_f64 * t35152;
    let t37217 = 0.55331893559454114829e-8_f64 * t35155;
    let t37218 = 0.99044544404633838508e-5_f64 * t35157;
    (t37205, t37206, t37207, t37208, t37210, t37211, t37212, t37213, t37214, t37216, t37217, t37218)
}
