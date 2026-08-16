//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 979/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk979<F: Float>(t1447: F, t6509: F, t5499: F, t6513: F, t486: F, t6610: F, t5115: F, t802: F, t12981: F, t6633: F, t13007: F, t6562: F) -> (F, F, F, F, F, F) {
    let t16522 = t1447 * t6509;
    let t16524 = t5499 * t6513;
    let t16535 = t486 * t6610;
    let t16537 = t802 * t5115;
    let t16542 = t12981 * t6633;
    let t16549 = t13007 * t6562;
    (t16522, t16524, t16535, t16537, t16542, t16549)
}
