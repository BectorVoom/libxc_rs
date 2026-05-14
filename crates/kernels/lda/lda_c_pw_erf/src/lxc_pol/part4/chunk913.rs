//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 913/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk913<F: Float>(t6125: F, t7060: F, t7079: F, t7305: F, t312: F, t19: F, t2686: F, t729: F, t734: F, t2696: F, t2699: F, t2702: F, t2708: F, t2711: F, t2738: F, t2747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7307 = t6125 + t7060 + t7079 + t7305;
    let t7308 = t7307 * t312;
    let t7314 = t2686 * t729 * t19;
    let t7315 = t7314 * t734;
    let t8097 = 1.8960024086108225 * t2696;
    let t8098 = 0.06506148529668915 * t2699;
    let t8099 = 1.9263778438055648 * t2702;
    let t8101 = 0.1301229705933783 * t2708;
    let t8102 = 0.08674864706225219 * t2711;
    let t8103 = 2.339289358982082 * t2738;
    let t8106 = 3.436685857643691 * t2747;
    (t7307, t7308, t7314, t7315, t8097, t8098, t8099, t8101, t8102, t8103, t8106)
}
