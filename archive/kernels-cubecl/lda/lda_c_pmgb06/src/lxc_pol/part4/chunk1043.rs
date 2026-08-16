//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1043/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1043<F: Float>(t1347: F, t718: F, t10637: F, t118: F, t3982: F, t1139: F, t415: F, t1135: F, t117: F, t123: F, t191: F, t4001: F) -> (F, F, F, F, F, F) {
    let t10864 = F::cast_from(0.3780648866776934_f64) * t718 * t1347;
    let t10866 = F::cast_from(0.0014238371845981686_f64) * t10637 * t118;
    let t10867 = t3982 * t118;
    let t10869 = t1139 * t415;
    let t10876 = F::cast_from(0.7561297733553868_f64) * t1135 * t415;
    let t10886 = F::cast_from(0.4097848972398244_f64) * t123 * t4001 * t191 * t117;
    (t10864, t10866, t10867, t10869, t10876, t10886)
}
