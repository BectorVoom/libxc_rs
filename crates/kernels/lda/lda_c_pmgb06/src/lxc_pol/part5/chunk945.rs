//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 945/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk945<F: Float>(t13560: F, t2085: F, t2060: F, t848: F, t2082: F, t955: F, t2079: F, t1554: F, t161: F, t2094: F, t199: F, t5575: F) -> (F, F, F, F, F, F) {
    let t14162 = t13560 * t2085;
    let t14170 = t2060 * t848;
    let t14181 = t955 * t2082;
    let t14183 = t955 * t2079;
    let t14211 = t161 * t1554 * t2094;
    let t14212 = t14211 / F::cast_from(45.0_f64);
    let t14231 = t5575 * t199;
    (t14162, t14170, t14181, t14183, t14212, t14231)
}
