//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1118/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1118<F: Float>(t1397: F, t1887: F, t3060: F, t802: F, t161: F, t4839: F, t497: F, t512: F, t10099: F, t10101: F, t10103: F, t10105: F) -> (F, F, F, F, F, F, F) {
    let t13277 = t1887 * t1397 / F::new(5.0);
    let t13279 = t802 * t3060 / F::new(10.0);
    let t13283 = F::new(2.0) / F::new(15.0) * t161 * t4839 * t512 * t497;
    let t13284 = F::new(2.0) / F::new(81.0) * t10099;
    let t13285 = F::new(4.0) / F::new(27.0) * t10101;
    let t13286 = F::new(2.0) / F::new(45.0) * t10103;
    let t13287 = F::new(2.0) / F::new(27.0) * t10105;
    (t13277, t13279, t13283, t13284, t13285, t13286, t13287)
}
