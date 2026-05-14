//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1016/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1016<F: Float>(t5272: F, t7561: F, t5277: F, t1181: F, t4665: F, t7351: F, t7564: F, t30219: F, t8469: F, t4752: F, t604: F, t7575: F, t4762: F, t1562: F, t31824: F, t1449: F, t30148: F, t30159: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t35766 = t7561 * t5272;
    let t35768 = t7561 * t5277;
    let t35772 = t7564 * t1181 * t7351 * t4665;
    let t35774 = t30219 * t8469;
    let t35775 = 0.31448092289604152068e-2 * t35774;
    let t35778 = t7575 * t1181 * t604 * t4752;
    let t35782 = t7564 * t1181 * t7351 * t4762;
    let t35784 = t31824 * t1562;
    let t35785 = 0.34299214494455789578e-2 * t35784;
    let t35788 = t30159 * t7586 * t30148 * t1449;
    (t35766, t35768, t35772, t35775, t35778, t35782, t35785, t35788)
}
