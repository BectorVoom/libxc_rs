//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1042/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1042<F: Float>(t1986: F, t6814: F, t559: F, t6326: F, t544: F, t6830: F, t1990: F, t6632: F, t603: F, t6735: F, t75: F, t22120: F, t22598: F, t22601: F, t601: F) -> (F, F, F, F, F, F) {
    let t22680 = t1986 * t6814;
    let t22681 = F::new(0.4155781415850207192e3) * t22680;
    let t22682 = t6326 * t559;
    let t22683 = F::new(480.0) * t22682;
    let t22684 = t544 * t6830;
    let t22685 = F::new(48.0) * t22684;
    let t22686 = t6632 * t1990;
    let t22687 = F::new(0.70178680769462448852e1) * t22686;
    let t22689 = t6735 * t75 * t603;
    let t22690 = F::new(0.23392893589820816284e1) * t22689;
    let t22694 = F::new(0.91080982599109921211e5) * t601 * t22598 * t22120 * t22601;
    (t22681, t22683, t22685, t22687, t22690, t22694)
}
