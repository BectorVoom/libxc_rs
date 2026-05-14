//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 942/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk942<F: Float>(t16213: F, t16215: F, t16217: F, t16219: F, t12252: F, t132: F, t137: F, t2604: F, t486: F, t7618: F, t14348: F, t14350: F, t14357: F, t14359: F, t19736: F, t19738: F) -> (F, F, F, F, F, F, F) {
    let t19739 = 4.0 / 45.0 * t16213;
    let t19740 = 8.0 / 45.0 * t16215;
    let t19741 = 4.0 / 27.0 * t16217;
    let t19742 = 8.0 / 27.0 * t16219;
    let t19746 = t132 * t137 * t12252 * t2604 / 5.0;
    let t19748 = t486 * t7618 / 30.0;
    let t19751 = -t19736 - t19738 - t19739 - t19740 + t19741 + t19742 + t19746 + t19748 + t14348 + 0.10063568466999305 * t14350 + t14357 + 0.9738937226128359 * t14359;
    (t19739, t19740, t19741, t19742, t19746, t19748, t19751)
}
