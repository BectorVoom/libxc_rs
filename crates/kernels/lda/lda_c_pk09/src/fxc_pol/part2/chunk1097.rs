//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1097/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1097<F: Float>(t12058: F, t12072: F, t1776: F, t452: F, t1971: F, t2939: F, t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t6633: F, t6634: F, t6638: F) -> (F, F, F) {
    let t12073 = t12058 + t12072;
    let t12074 = t12073 * t1776;
    let t12075 = t12074 * t452;
    let t12082 = t2939 * t1971;
    let t12099 = F::new(0.505765839233979) * t11066 + F::new(1.011531678467958) * t10959 + F::new(4.0) * t11529 - F::new(4.0) * t11532 - F::new(4.0) * t11535 + F::new(6.0) * t11539 - F::new(4.0) * t11542 + F::new(0.505765839233979) * t11076 + t6633 + F::new(0.168588613077993) * t11073 + t6638 - F::new(0.168588613077993) * t6337 - F::new(0.505765839233979) * t6323 + F::new(1.3333333333333333) * t6550 + t6634 - F::new(1.3333333333333333) * t6508 + F::new(0.168588613077993) * t6467;
    (t12075, t12082, t12099)
}
