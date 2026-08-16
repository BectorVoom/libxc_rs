//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1042/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1042(t1986: f64, t6814: f64, t559: f64, t6326: f64, t544: f64, t6830: f64, t1990: f64, t6632: f64, t603: f64, t6735: f64, t75: f64, t22120: f64, t22598: f64, t22601: f64, t601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22680 = t1986 * t6814;
    let t22681 = 0.4155781415850207192e3_f64 * t22680;
    let t22682 = t6326 * t559;
    let t22683 = 480.0_f64 * t22682;
    let t22684 = t544 * t6830;
    let t22685 = 48.0_f64 * t22684;
    let t22686 = t6632 * t1990;
    let t22687 = 0.70178680769462448852e1_f64 * t22686;
    let t22689 = t6735 * t75 * t603;
    let t22690 = 0.23392893589820816284e1_f64 * t22689;
    let t22694 = 0.91080982599109921211e5_f64 * t601 * t22598 * t22120 * t22601;
    (t22681, t22683, t22685, t22687, t22690, t22694)
}
