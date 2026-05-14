//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 682/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk682<F: Float>(t2079: F, t405: F, t1576: F, t4880: F, t3358: F, t4885: F, t103: F, t2060: F, t4848: F, t4853: F, t4859: F, t4863: F, t4868: F, t4871: F, t4874: F, t4876: F, t4879: F, t4882: F, t4887: F, t4889: F, t4892: F, t4896: F) -> (F, F, F) {
    let t4898 = 0.002962962962962963 * t405 * t2079;
    let t4899 = t1576 * t4880;
    let t4902 = t3358 * t4885;
    let t4905 = 0.013333333333333334 * t103 * t4848 - 0.05333333333333334 * t2060 * t4853 + 0.14396666666666666 * t4859 - 0.21595 * t4863 + 0.09597777777777777 * t4868 + 0.07198333333333333 * t4871 - 0.2879333333333333 * t4874 - 0.047988888888888886 * t4876 + t4879 - 0.023994444444444443 * t4882 - 0.03999074074074074 * t4887 + 0.013333333333333334 * t103 * t4889 - 0.04 * t103 * t4892 - t4896 + t4898 - 0.0022222222222222222 * t103 * t4899 - 0.002962962962962963 * t103 * t4902;
    (t4899, t4902, t4905)
}
