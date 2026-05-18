//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 998/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk998<F: Float>(t11866: F, t436: F, t4754: F, t1559: F, t439: F, t4779: F, t2002: F, t3186: F, t1925: F, t3226: F, t1600: F, t1988: F) -> (F, F, F, F, F, F) {
    let t11867 = F::new(2.0) / F::new(135.0) * t11866;
    let t11868 = t4754 * t436;
    let t11869 = t11868 / F::new(15.0);
    let t11872 = F::new(2.0) / F::new(15.0) * t439 * t4779 * t1559;
    let t11874 = F::new(2.0) / F::new(15.0) * t2002 * t3186;
    let t11875 = t3226 * t1925;
    let t11876 = F::new(4.0) / F::new(45.0) * t11875;
    let t11877 = t1988 * t1600;
    (t11867, t11869, t11872, t11874, t11876, t11877)
}
