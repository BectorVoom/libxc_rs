//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1117/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1117<F: Float>(t2480: F, t3216: F, t439: F, t1426: F, t6244: F, t2485: F, t3177: F, t1420: F, t6250: F, t10255: F, t2484: F, t12908: F, t12913: F, t12915: F, t12917: F, t12919: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16724 = t439 * t3216 * t2480 / 45.0;
    let t16727 = 2.0 / 45.0 * t439 * t1426 * t6244;
    let t16729 = t3177 * t2485 / 27.0;
    let t16731 = 2.0 / 27.0 * t1420 * t6250;
    let t16734 = t439 * t10255 * t2484 / 27.0;
    let t16735 = 8.0 / 45.0 * t12908;
    let t16736 = 8.0 / 135.0 * t12913;
    let t16737 = 8.0 / 135.0 * t12915;
    let t16738 = 4.0 / 135.0 * t12917;
    let t16739 = 4.0 / 81.0 * t12919;
    (t16724, t16727, t16729, t16731, t16734, t16735, t16736, t16737, t16738, t16739)
}
