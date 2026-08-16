//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1043/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1043(t1347: f64, t718: f64, t10637: f64, t118: f64, t3982: f64, t1139: f64, t415: f64, t1135: f64, t117: f64, t123: f64, t191: f64, t4001: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10864 = 0.3780648866776934_f64 * t718 * t1347;
    let t10866 = 0.0014238371845981686_f64 * t10637 * t118;
    let t10867 = t3982 * t118;
    let t10869 = t1139 * t415;
    let t10876 = 0.7561297733553868_f64 * t1135 * t415;
    let t10886 = 0.4097848972398244_f64 * t123 * t4001 * t191 * t117;
    (t10864, t10866, t10867, t10869, t10876, t10886)
}
