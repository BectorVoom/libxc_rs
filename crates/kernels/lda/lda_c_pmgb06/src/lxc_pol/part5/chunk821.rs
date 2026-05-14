//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 821/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk821<F: Float>(t2791: F, t391: F, t1329: F, t1347: F, t718: F, t10637: F, t118: F, t1139: F, t415: F, t1135: F, t117: F, t123: F, t191: F, t4001: F, t1100: F, t290: F) -> (F, F, F, F, F, F, F, F) {
    let t10860 = 0.12602162889256446 * t391 * t2791;
    let t10861 = t1329 * t1347;
    let t10864 = 0.3780648866776934 * t718 * t1347;
    let t10866 = 0.0014238371845981686 * t10637 * t118;
    let t10869 = t1139 * t415;
    let t10876 = 0.7561297733553868 * t1135 * t415;
    let t10886 = 0.4097848972398244 * t123 * t4001 * t191 * t117;
    let t10895 = 6.399008129061525 * t1100 * t290;
    (t10860, t10861, t10864, t10866, t10869, t10876, t10886, t10895)
}
