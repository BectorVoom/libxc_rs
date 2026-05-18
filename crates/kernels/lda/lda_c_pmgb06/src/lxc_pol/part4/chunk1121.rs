//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1121/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1121<F: Float>(t1179: F, t1798: F, t419: F, t421: F, t1186: F, t5613: F, t1354: F, t2841: F, t4429: F, t118: F, t5567: F, t11676: F) -> (F, F, F, F, F) {
    let t14297 = t1179 * t1798 * t419 * t421;
    let t14300 = t5613 * t1186 * t421;
    let t14303 = t4429 * t2841 * t1354;
    let t14306 = t5567 * t118;
    let t14308 = t11676 * t118;
    (t14297, t14300, t14303, t14306, t14308)
}
