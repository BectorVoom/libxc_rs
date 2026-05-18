//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1001/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1001<F: Float>(t1435: F, t2475: F, t2497: F, t1342: F, t9836: F, t1307: F, t10186: F, t1476: F, t1215: F, t2689: F, t2629: F, t2667: F) -> (F, F, F, F, F, F, F, F) {
    let t10823 = t2475 * t1435;
    let t10825 = t2497 * t1435;
    let t10827 = t1342 * t9836;
    let t10829 = t1307 * t9836;
    let t10841 = t1476 * t10186;
    let t10843 = t2689 * t1215;
    let t10846 = t2629 * t1435;
    let t10848 = t2667 * t1435;
    (t10823, t10825, t10827, t10829, t10841, t10843, t10846, t10848)
}
