//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1188/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1188<F: Float>(t1186: F, t421: F, t5617: F, t2329: F, t2837: F, t1179: F, t1798: F, t419: F, t5613: F, t1354: F, t2841: F, t4429: F) -> (F, F, F, F, F) {
    let t14290 = t5617 * t1186 * t421;
    let t14291 = F::cast_from(0.01185233419734569_f64) * t14290;
    let t14293 = t2329 * t2837 * t421;
    let t14297 = t1179 * t1798 * t419 * t421;
    let t14298 = F::cast_from(0.01975389032890948_f64) * t14297;
    let t14300 = t5613 * t1186 * t421;
    let t14303 = t4429 * t2841 * t1354;
    (t14291, t14293, t14298, t14300, t14303)
}
