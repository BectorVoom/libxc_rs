//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 655/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk655<F: Float>(t1795: F, t415: F, t117: F, t123: F, t740: F, t859: F, t2209: F, t73: F, t1282: F, t2229: F, t365: F, t110: F, t30: F, t342: F) -> (F, F, F, F, F, F) {
    let t5702 = t1795 * t415;
    let t5712 = t123 * t740 * t859 * t117;
    let t5721 = t73 * t2209;
    let t5740 = t1282 * t2209;
    let t5770 = t365 * t2229;
    let t5772 = t30 * t110 * t342;
    (t5702, t5712, t5721, t5740, t5770, t5772)
}
