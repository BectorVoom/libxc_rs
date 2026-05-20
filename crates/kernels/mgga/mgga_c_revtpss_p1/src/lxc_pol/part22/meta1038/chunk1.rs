//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3629/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629<F: Float>(t16784: F, t5198: F, t12571: F, t6548: F, t1149: F, t56265: F, t57795: F, t1196: F, t17151: F, t5197: F, t16639: F, t5192: F) -> (F, F, F, F, F) {
    let t68725 = F::cast_from(0.46785788981077169656e1_f64) * t16784 * t5198;
    let t68727 = F::cast_from(0.11696447245269292414e1_f64) * t12571 * t6548;
    let t68730 = F::cast_from(0.2069040516770936012e4_f64) * t57795 * t56265 * t1149;
    let t68733 = F::cast_from(0.23392894490538584828e1_f64) * t1196 * t5197 * t17151;
    let t68735 = F::cast_from(0.70178683471615754484e1_f64) * t5192 * t16639;
    (t68725, t68727, t68730, t68733, t68735)
}
