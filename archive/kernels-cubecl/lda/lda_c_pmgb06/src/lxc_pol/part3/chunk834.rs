//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 834/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk834<F: Float>(t153: F, t1962: F, t4619: F, t136: F, t813: F, t1601: F, t497: F, t1593: F, t443: F, t176: F, t1988: F, t4588: F) -> (F, F, F, F, F, F, F) {
    let t6494 = t1962 * t153;
    let t6498 = t4619 * t153;
    let t6550 = t136 * t813;
    let t6559 = t1601 * t497;
    let t6636 = t1593 * t443;
    let t6747 = t1988 * t176;
    let t6751 = t4588 * t176;
    (t6494, t6498, t6550, t6559, t6636, t6747, t6751)
}
