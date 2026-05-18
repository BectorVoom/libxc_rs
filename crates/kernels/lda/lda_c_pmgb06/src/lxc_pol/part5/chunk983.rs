//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 983/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk983<F: Float>(t2466: F, t3223: F, t161: F, t489: F, t6905: F, t5326: F, t802: F, t432: F, t6621: F, t1554: F, t2653: F, t1848: F, t2018: F) -> (F, F, F, F, F, F) {
    let t16866 = t3223 * t2466;
    let t16869 = t161 * t489 * t6905;
    let t16875 = t802 * t5326;
    let t16877 = t432 * t6621;
    let t16880 = t161 * t1554 * t2653;
    let t16884 = t1848 * t2018;
    (t16866, t16869, t16875, t16877, t16880, t16884)
}
