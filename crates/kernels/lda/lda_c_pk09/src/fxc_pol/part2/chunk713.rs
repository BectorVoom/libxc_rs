//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 713/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk713<F: Float>(t309: F, t454: F, t4977: F, t2040: F, t6791: F, t6803: F, t2037: F, t6253: F, t2056: F, t7017: F, t633: F, t6611: F) -> (F, F, F, F, F, F, F, F) {
    let t7030 = t309 * t454 * t4977;
    let t7032 = t2040 * t7030 / F::new(6.0);
    let t7041 = F::new(0.037002892246025966) * t6791;
    let t7045 = F::new(0.14975624337724558) * t6803;
    let t7049 = t2037 * t6253;
    let t7053 = t2056 * t7030 / F::new(6.0);
    let t7064 = t2040 * t7017 / F::new(9.0);
    let t7066 = t309 * t6611 * t633;
    (t7030, t7032, t7041, t7045, t7049, t7053, t7064, t7066)
}
