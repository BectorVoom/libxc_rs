//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 654/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk654<F: Float>(t545: F, t5655: F, t1366: F, t2349: F, t187: F, t2342: F, t2345: F, t1799: F, t415: F, t1347: F, t795: F, t118: F, t5522: F) -> (F, F, F, F, F, F, F) {
    let t5656 = t5655 * t545;
    let t5658 = t2349 * t1366;
    let t5674 = F::new(8.0) / F::new(3.0) * t2342 * t187;
    let t5675 = t2345 * t187;
    let t5697 = F::new(0.06301081444628223) * t1799 * t415;
    let t5698 = t795 * t1347;
    let t5701 = F::new(0.06301081444628223) * t5522 * t118;
    (t5656, t5658, t5674, t5675, t5697, t5698, t5701)
}
