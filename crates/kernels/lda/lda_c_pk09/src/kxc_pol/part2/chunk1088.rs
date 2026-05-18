//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1088/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1088<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t6882: F, t6889: F, t6890: F, t6895: F) -> F {
    let t11950 = F::new(0.4537481858318121) * t6527 - F::new(0.4537481858318121) * t6519 - F::new(0.22687409291590604) * t11556 + F::new(0.22687409291590604) * t11559 - F::new(0.015084944665313014) * t10962 + F::new(0.15124939527727072) * t11563 - F::new(0.15124939527727072) * t11566 - F::new(0.04525483399593904) * t11070 - F::new(0.04525483399593904) * t10954 - F::new(0.04525483399593904) * t10966 - F::new(0.04525483399593904) * t11062 + F::new(0.22687409291590604) * t11574 + F::new(0.04525483399593904) * t6327 + t6882 + t6889 - t6890 - t6895;
    t11950
}
