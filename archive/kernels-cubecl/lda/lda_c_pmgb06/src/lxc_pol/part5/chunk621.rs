//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 621/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk621<F: Float>(t1: F, t1464: F, t1827: F, t350: F, t1822: F, t2082: F, t405: F, t2079: F, t848: F, t955: F, t839: F, t947: F) -> (F, F, F, F, F, F, F, F) {
    let t4865 = t1464 * t1;
    let t4876 = t350 * t1827;
    let t4878 = t350 * t1822;
    let t4879 = F::cast_from(0.015996296296296297_f64) * t4878;
    let t4896 = F::cast_from(0.017777777777777778_f64) * t405 * t2082;
    let t4898 = F::cast_from(0.002962962962962963_f64) * t405 * t2079;
    let t4909 = t955 * t848;
    let t4911 = t947 * t839;
    (t4865, t4876, t4878, t4879, t4896, t4898, t4909, t4911)
}
