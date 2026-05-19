//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1204/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1204<F: Float>(t15872: F, t5077: F, t5084: F, t10679: F, t10681: F, t10684: F, t15836: F, t15839: F, t15841: F, t15843: F, t15849: F, t15851: F, t15857: F, t15860: F, t15864: F, t15867: F, t15870: F) -> (F, F) {
    let t15875 = F::new(4.0) / F::new(15.0) * t5077 * t5084 * t15872;
    let t15876 = t15836 + t15839 + t15841 + t15843 + t15849 + t15851 + F::cast_from(0.07214027574909895_f64) * t10679 - F::cast_from(0.022363485482220676_f64) * t10681 - t10684 + t15857 - t15860 + t15864 - t15867 - t15870 - t15875;
    (t15875, t15876)
}
