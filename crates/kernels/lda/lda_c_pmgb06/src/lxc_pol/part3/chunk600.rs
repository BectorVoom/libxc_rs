//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk600<F: Float>(t36: F, t28: F, t247: F, t3682: F, t950: F, t2781: F, t628: F, t1830: F, t2060: F, t3680: F, t3683: F, t3685: F, t633: F, t622: F, t959: F, t971: F) -> (F, F, F, F, F, F, F, F) {
    let t3688 = 1.0/pow_3_2(t36);
    let t3689 = t3688 * t28;
    let t3690 = t3689 * t247;
    let t3692 = t950 * t3682;
    let t3694 = t628 * t2781;
    let t3697 = -2.5319 * t3680 + 1.6879333333333333 * t3683 - 1.9692555555555555 * t3685 - 0.9301185185185186 * t1830 + 0.13651666666666668 * t3690 - 0.27303333333333335 * t3692 - 0.31853888888888887 * t3694 - 0.36514074074074077 * t2060;
    let t3698 = t3697 * t633;
    let t3700 = 1.0 * t622 * t3698;
    let t3701 = t971 * t959;
    (t3689, t3690, t3692, t3694, t3697, t3698, t3700, t3701)
}
