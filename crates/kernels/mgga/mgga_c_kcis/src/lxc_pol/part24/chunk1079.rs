//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1079/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1079<F: Float>(t1250: F, t251: F, t47323: F, t96217: F, t15216: F, t28101: F, t26960: F, t1268: F, t9494: F, t26955: F, t96261: F, t96270: F, t28171: F, t7784: F, t26966: F, t28160: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97297 = t47323 * t251 * t1250;
    let t97312 = 0.15476481481481481481e-2 * t96217;
    let t97330 = t15216 * t28101;
    let t97332 = 0.7722800925925925926e-4 * t26960 * t97330;
    let t97338 = t1268 * t9494;
    let t97344 = 0.10306077835648148148e-4 * t26955 * t97330;
    let t97352 = 0.61905925925925925925e-2 * t96261;
    let t97360 = 0.61905925925925925925e-2 * t96270;
    let t97377 = 0.61782407407407407408e-3 * t28171 * t7784;
    let t97385 = 0.61782407407407407408e-3 * t26966 * t28160;
    (t97297, t97312, t97332, t97338, t97344, t97352, t97360, t97377, t97385)
}
