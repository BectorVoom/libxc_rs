//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1036/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1036<F: Float>(t11869: F, t11872: F, t11874: F, t11876: F, t11880: F, t11882: F, t11885: F, t11886: F, t11889: F, t11890: F, t11891: F, t11892: F, t10679: F, t10681: F, t10684: F, t11893: F, t11894: F, t11895: F, t11898: F, t11902: F, t11906: F, t11910: F, t11912: F) -> (F, F) {
    let t14326 = t11869 - t11872 - t11874 - t11876 + t11880 + t11882 + t11885 + t11886 + t11889 - t11890 - t11891 - t11892;
    let t14330 = t11893 + t11894 + t11895 + t11898 + 0.21642082724729686 * t10679 - 0.03354522822333102 * t10681 - t10684 + t11902 + t11906 + t11910 - t11912;
    (t14326, t14330)
}
