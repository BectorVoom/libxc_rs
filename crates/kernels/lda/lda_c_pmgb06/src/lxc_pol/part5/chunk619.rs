//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 619/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk619<F: Float>(t1836: F, t489: F, t161: F, t1933: F, t486: F, t1835: F, t517: F, t1887: F, t436: F, t1928: F, t432: F, t1873: F, t435: F) -> (F, F, F, F, F, F, F) {
    let t4790 = t489 * t1836;
    let t4792 = F::new(2.0) / F::new(45.0) * t161 * t4790;
    let t4794 = F::new(2.0) / F::new(45.0) * t486 * t1933;
    let t4801 = t1835 * t517;
    let t4807 = F::new(2.0) / F::new(45.0) * t1887 * t436;
    let t4809 = F::new(2.0) / F::new(45.0) * t432 * t1928;
    let t4810 = t435 * t1873;
    (t4790, t4792, t4794, t4801, t4807, t4809, t4810)
}
