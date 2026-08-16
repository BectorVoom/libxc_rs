//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 994/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk994<F: Float>(t1504: F, t2594: F, t10020: F, t1319: F, t2507: F, t5267: F, t306: F, t1329: F, t309: F, t310: F, t2648: F, t5248: F) -> (F, F, F, F, F) {
    let t10703 = t2594 * t1504;
    let t10706 = t1319 * t10020;
    let t10712 = t2507 * t5267;
    let t10713 = t10712 * t306;
    let t10715 = t309 * t310 * t1329;
    let t10719 = t2648 * t5248;
    (t10703, t10706, t10713, t10715, t10719)
}
