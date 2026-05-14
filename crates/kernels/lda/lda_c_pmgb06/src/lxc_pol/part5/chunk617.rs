//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 617/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk617<F: Float>(t1377: F, t5649: F, t2342: F, t27: F, t545: F, t2345: F, t1366: F, t2349: F, t187: F, t1799: F, t415: F, t1347: F, t795: F, t118: F, t5522: F, t1795: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5650 = t5649 * t1377;
    let t5652 = t2342 * t27;
    let t5654 = 0.21642082724729686 * t5652 * t545;
    let t5655 = t2345 * t27;
    let t5656 = t5655 * t545;
    let t5658 = t2349 * t1366;
    let t5674 = 8.0 / 3.0 * t2342 * t187;
    let t5675 = t2345 * t187;
    let t5697 = 0.06301081444628223 * t1799 * t415;
    let t5698 = t795 * t1347;
    let t5701 = 0.06301081444628223 * t5522 * t118;
    let t5702 = t1795 * t415;
    (t5650, t5652, t5654, t5655, t5656, t5658, t5674, t5675, t5697, t5698, t5701, t5702)
}
