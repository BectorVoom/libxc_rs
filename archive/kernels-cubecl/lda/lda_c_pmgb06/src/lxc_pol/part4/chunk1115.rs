//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1115/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1115<F: Float>(t1392: F, t1887: F, t3068: F, t802: F, t161: F, t489: F, t4940: F, t3050: F, t405: F, t4892: F, t4889: F, t4902: F) -> (F, F, F, F, F, F, F) {
    let t14017 = t1887 * t1392;
    let t14019 = t802 * t3068;
    let t14024 = t161 * t489 * t4940;
    let t14068 = t802 * t3050;
    let t14073 = t405 * t4892;
    let t14078 = t405 * t4889;
    let t14080 = t405 * t4902;
    (t14017, t14019, t14024, t14068, t14073, t14078, t14080)
}
