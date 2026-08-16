//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1098/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1098<F: Float>(t161: F, t489: F, t5416: F, t1499: F, t1933: F, t4790: F, t486: F, t1447: F, t5359: F, t1902: F, t3213: F, t5494: F) -> (F, F, F, F, F, F) {
    let t13235 = t161 * t489 * t5416;
    let t13237 = t1499 * t1933;
    let t13239 = t486 * t4790;
    let t13241 = t1447 * t5359;
    let t13243 = t3213 * t1902;
    let t13245 = t1447 * t5494;
    (t13235, t13237, t13239, t13241, t13243, t13245)
}
