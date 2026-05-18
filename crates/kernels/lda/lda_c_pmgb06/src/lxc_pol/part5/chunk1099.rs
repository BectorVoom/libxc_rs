//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1099/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1099<F: Float>(t9762: F, t9765: F, t16593: F, t6616: F, t831: F, t486: F, t7726: F, t12840: F, t161: F, t166: F, t2599: F, t6232: F) -> (F, F, F, F, F, F, F) {
    let t20209 = F::new(4.0) / F::new(405.0) * t9762;
    let t20210 = F::new(4.0) / F::new(405.0) * t9765;
    let t20211 = t16593 / F::new(45.0);
    let t20212 = t831 * t6616;
    let t20213 = t20212 / F::new(15.0);
    let t20215 = t486 * t7726 / F::new(5.0);
    let t20219 = t161 * t166 * t12840 * t2599 / F::new(5.0);
    let t20221 = t831 * t6232 / F::new(10.0);
    (t20209, t20210, t20211, t20213, t20215, t20219, t20221)
}
