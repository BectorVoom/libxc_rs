//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1192/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1192<F: Float>(t11837: F, t11842: F, t11843: F, t11845: F, t11846: F, t11847: F, t11853: F, t11859: F, t11861: F, t11865: F, t11867: F, t11869: F, t11872: F, t11874: F, t11876: F, t11880: F, t11882: F, t11885: F, t11886: F, t11889: F, t11890: F, t11891: F, t11892: F) -> (F, F) {
    let t14325 = -t11837 - t11842 + t11843 - t11845 + t11846 - t11847 + t11853 + t11859 - t11861 + t11865 - t11867;
    let t14326 = t11869 - t11872 - t11874 - t11876 + t11880 + t11882 + t11885 + t11886 + t11889 - t11890 - t11891 - t11892;
    (t14325, t14326)
}
