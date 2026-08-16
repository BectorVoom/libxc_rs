//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1092/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1092<F: Float>(t3259: F, t5066: F, t5075: F, t154: F, t3092: F, t465: F, t1395: F, t1438: F, t12535: F, t441: F, t12683: F, t5082: F) -> (F, F, F, F, F, F) {
    let t13026 = t5075 * t5066 * t3259;
    let t13027 = t154 * t3092;
    let t13031 = t465 * t3092;
    let t13035 = t1395 * t1438;
    let t13043 = t5075 * t12535 * t441;
    let t13047 = t12683 * t5082;
    (t13026, t13027, t13031, t13035, t13043, t13047)
}
