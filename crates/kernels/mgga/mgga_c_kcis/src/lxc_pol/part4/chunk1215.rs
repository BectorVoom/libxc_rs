//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1215/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1215<F: Float>(t12119: F, t12858: F, t1592: F, t16656: F, t16661: F, t16663: F, t16668: F, t16676: F, t16679: F, t16688: F, t16697: F, t16702: F, t16704: F, t16706: F, t16708: F, t17731: F, t17969: F, t17981: F, t4409: F, t4414: F, t6208: F) -> (F,) {
    let t18034 = -0.11607361111111111111e-2 * t16656 - 0.38691203703703703703e-3 * t16661 - 0.51588271604938271604e-3 * t16663 + 0.69644166666666666664e-2 * t16668 - 0.23214722222222222222e-2 * t16676 - 0.61905925925925925924e-2 * t16679 + 0.13345e0 * t1592 * t17731 + 0.19345601851851851852e-2 * t16688 + 0.51588271604938271605e-2 * t16697 - 0.15476481481481481481e-2 * t12119 - 0.23214722222222222222e-2 * t16702 + 0.890445125e-2 * t4414 * t17969 - 0.178244852896875e-2 * t12858 * t17981 + 0.13345e0 * t4409 * t6208 - 0.13345e0 * t1592 * t17981 - 0.23214722222222222222e-2 * t16704 + 0.15476481481481481481e-2 * t16706 - 0.46429444444444444444e-2 * t16708;
    (t18034,)
}
