//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1186/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1186<F: Float>(t12990: F, t12998: F, t12025: F, t12362: F, t15825: F, t16822: F, t3965: F, t4494: F, t12113: F, t17083: F, t4488: F, t12439: F, t15607: F, t4509: F, t12956: F, t5295: F) -> (F, F, F, F, F, F, F, F) {
    let t17525 = 32.0 / 45.0 * t12990;
    let t17526 = 32.0 / 135.0 * t12998;
    let t17529 = 64.0 / 9.0 * t12362 * t12025 * t15825;
    let t17532 = 32.0 / 45.0 * t3965 * t4494 * t16822;
    let t17535 = 16.0 / 15.0 * t4488 * t12113 * t17083;
    let t17538 = 16.0 / 9.0 * t4488 * t12439 * t17083;
    let t17540 = 32.0 / 45.0 * t15607 * t4509;
    let t17543 = 32.0 / 45.0 * t3965 * t12956 * t5295;
    (t17525, t17526, t17529, t17532, t17535, t17538, t17540, t17543)
}
