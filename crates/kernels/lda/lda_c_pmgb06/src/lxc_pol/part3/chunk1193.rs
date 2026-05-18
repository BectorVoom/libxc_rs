//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1193/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1193<F: Float>(t10679: F, t10681: F, t10684: F, t11893: F, t11894: F, t11895: F, t11898: F, t11902: F, t11906: F, t11910: F, t11912: F, t11915: F, t11918: F, t11934: F, t11937: F, t11940: F, t11943: F, t11946: F, t11951: F, t11953: F, t11955: F, t11959: F, t11970: F) -> (F, F) {
    let t14330 = t11893 + t11894 + t11895 + t11898 + F::new(0.21642082724729686) * t10679 - F::new(0.03354522822333102) * t10681 - t10684 + t11902 + t11906 + t11910 - t11912;
    let t14331 = -t11915 - t11918 + t11934 - t11937 - t11940 - t11943 - t11946 - t11951 - t11953 + t11955 + t11959 - t11970;
    (t14330, t14331)
}
