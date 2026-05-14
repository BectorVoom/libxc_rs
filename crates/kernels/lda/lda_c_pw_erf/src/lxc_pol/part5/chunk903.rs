//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 903/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk903<F: Float>(t3802: F, t519: F, t6326: F, t3859: F, t6331: F, t511: F, t7016: F, t1518: F, t2479: F, t548: F, t568: F, t6671: F, t184: F, t509: F, t784: F, t1982: F, t2134: F) -> (F, F, F, F, F, F, F) {
    let t16952 = t519 * t3802 * t6326;
    let t16955 = t519 * t3859 * t6331;
    let t16957 = t511 * t7016;
    let t16961 = t548 * t1518 * t2479;
    let t16963 = t6671 * t568;
    let t16971 = t784 * t509 * t184;
    let t16989 = t1982 * t2134;
    (t16952, t16955, t16957, t16961, t16963, t16971, t16989)
}
