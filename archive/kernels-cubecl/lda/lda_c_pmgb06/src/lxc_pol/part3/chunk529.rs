//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 529/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk529<F: Float>(t1322: F, t789: F, t73: F, t769: F, t388: F, t118: F, t1795: F, t415: F, t795: F, t409: F, t794: F, t419: F, t421: F) -> (F, F, F, F, F, F, F) {
    let t2308 = t789 * t1322;
    let t2311 = t73 * t769;
    let t2312 = t388 * t2311;
    let t2323 = t1795 * t118;
    let t2327 = t795 * t415;
    let t2329 = t409 * t794;
    let t2331 = t2329 * t419 * t421;
    (t2308, t2311, t2312, t2323, t2327, t2329, t2331)
}
