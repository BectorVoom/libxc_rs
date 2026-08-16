//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 756/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk756<F: Float>(t2061: F, t4913: F, t1619: F, t4650: F, t4663: F, t473: F, t4659: F, t3404: F, t4645: F, t4655: F, t4672: F, t4668: F) -> (F, F, F, F, F, F, F, F) {
    let t5006 = t4913 * t2061;
    let t5010 = t1619 * t4650;
    let t5013 = t473 * t4663;
    let t5016 = t1619 * t4659;
    let t5019 = t3404 * t4645;
    let t5022 = t1619 * t4655;
    let t5025 = t473 * t4672;
    let t5028 = t473 * t4668;
    (t5006, t5010, t5013, t5016, t5019, t5022, t5025, t5028)
}
