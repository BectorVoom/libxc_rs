//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1070/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1070<F: Float>(t1971: F, t2855: F, t1672: F, t2838: F, t11243: F, t6991: F, t6995: F, t6997: F, t7008: F, t7015: F, t7019: F, t7026: F, t7028: F, t7032: F) -> (F, F) {
    let t11600 = t2855 * t1971;
    let t11607 = t2838 * t1672;
    let t11610 = -t6991 + t11607 / F::new(18.0) - F::new(0.04991874779241519) * t11243 + t6995 + t6997 - t7008 - t7015 - t7019 + t7026 + t7028 + t7032;
    (t11600, t11610)
}
