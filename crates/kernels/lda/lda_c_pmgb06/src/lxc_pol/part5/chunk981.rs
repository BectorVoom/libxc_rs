//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 981/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk981<F: Float>(t132: F, t435: F, t6735: F, t6442: F, t1423: F, t6465: F, t6475: F, t2477: F, t3220: F, t6300: F, t5211: F, t6303: F) -> (F, F, F, F, F, F, F) {
    let t16605 = t132 * t435 * t6735;
    let t16612 = t132 * t435 * t6442;
    let t16687 = t1423 * t6465;
    let t16689 = t1423 * t6475;
    let t16697 = t3220 * t2477;
    let t16699 = t1423 * t6300;
    let t16701 = t5211 * t6303;
    (t16605, t16612, t16687, t16689, t16697, t16699, t16701)
}
