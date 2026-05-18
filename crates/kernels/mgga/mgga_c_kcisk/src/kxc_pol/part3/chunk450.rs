//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 450/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk450<F: Float>(t3532: F, t459: F, t3278: F, t3530: F, t1337: F, t306: F, t1163: F, t1175: F, t1422: F, t425: F, t1364: F, t1390: F) -> (F, F, F, F, F, F, F) {
    let t3533 = t459 * t3532;
    let t3535 = t3530 * t3533 * t3278;
    let t3538 = t1337 * t306;
    let t3539 = t3538 * t459;
    let t3540 = t1163 * t1175;
    let t3541 = t3539 * t3540;
    let t3544 = t1422 * t425;
    let t3545 = t1163 * t1364;
    let t3546 = t3544 * t3545;
    let t3549 = t459 * t1390;
    (t3533, t3535, t3539, t3541, t3544, t3546, t3549)
}
