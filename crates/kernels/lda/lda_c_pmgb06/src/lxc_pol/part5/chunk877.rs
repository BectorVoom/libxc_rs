//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 877/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk877<F: Float>(t5102: F, t831: F, t432: F, t6626: F, t13715: F, t153: F, t2501: F, t3213: F, t464: F, t6123: F, t1447: F, t6509: F, t5499: F, t6513: F, t486: F, t6610: F) -> (F, F, F, F, F, F, F, F) {
    let t16448 = t831 * t5102;
    let t16455 = t432 * t6626;
    let t16491 = t13715 * t153;
    let t16506 = t3213 * t2501;
    let t16513 = t6123 * t464;
    let t16522 = t1447 * t6509;
    let t16524 = t5499 * t6513;
    let t16535 = t486 * t6610;
    (t16448, t16455, t16491, t16506, t16513, t16522, t16524, t16535)
}
