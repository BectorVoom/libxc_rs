//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 812/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk812<F: Float>(t175: F, t3456: F, t132: F, t3034: F, t435: F, t152: F, t3030: F, t1623: F, t955: F, t3415: F, t405: F, t1620: F, t3408: F, t3405: F, t134: F, t147: F, t1531: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9636 = 1.0 / t3456 / t175;
    let t9644 = t132 * t435 * t3034;
    let t9647 = 1.0 / t3030 / t152;
    let t9679 = t955 * t1623;
    let t9681 = t405 * t3415;
    let t9683 = t955 * t1620;
    let t9685 = t405 * t3408;
    let t9687 = t405 * t3405;
    let t9693 = t147 / t134 / t1531;
    (t9636, t9644, t9647, t9679, t9681, t9683, t9685, t9687, t9693)
}
