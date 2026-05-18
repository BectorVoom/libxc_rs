//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1001/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1001<F: Float>(t130: F, t485: F, t5067: F, t5091: F, t1381: F, t3038: F, t5068: F, t851: F, t432: F, t5041: F, t11889: F, t11890: F, t11891: F, t11892: F, t11893: F, t11894: F, t11895: F, t11898: F, t11902: F) -> (F, F, F, F, F, F) {
    let t11903 = t485 * t130;
    let t11904 = t11903 * t5067;
    let t11906 = F::new(4.0) / F::new(15.0) * t11904 * t5091;
    let t11910 = F::new(4.0) / F::new(15.0) * t5068 * t3038 * t851 * t1381;
    let t11912 = t432 * t5041 / F::new(10.0);
    let t11913 = t11889 - t11890 - t11891 - t11892 + t11893 + t11894 + t11895 + t11898 + t11902 + t11906 + t11910 - t11912;
    (t11903, t11904, t11906, t11910, t11912, t11913)
}
