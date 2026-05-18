//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1004/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1004<F: Float>(t4159: F, t871: F, t9402: F, t11589: F, t11915: F, t11918: F, t11921: F, t11928: F, t11930: F, t11934: F, t11937: F, t11940: F, t11943: F, t205: F, t208: F, t213: F) -> (F, F) {
    let t11944 = t871 * t4159;
    let t11946 = t9402 / F::new(45.0);
    let t11947 = -t11915 - t11918 + t11921 + t11589 * t205 * t208 * t213 / F::new(3.0) + t11928 + F::new(0.18233333333333332) * t11930 + t11934 - t11937 - t11940 - t11943 - F::new(0.06649088888888889) * t11944 - t11946;
    (t11946, t11947)
}
