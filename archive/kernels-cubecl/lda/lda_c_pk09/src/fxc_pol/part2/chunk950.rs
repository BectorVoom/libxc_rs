//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 950/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk950<F: Float>(t409: F, t9927: F, t6041: F, t5785: F, t9578: F, t15: F, t309: F, t9581: F, t1494: F, t2594: F, t5777: F, t1504: F, t310: F) -> (F, F, F, F, F, F) {
    let t10000 = t409 * t9927;
    let t10001 = t6041 * t10000;
    let t10003 = t9578 * t5785;
    let t10004 = t309 * t15;
    let t10005 = t10004 * t9581;
    let t10010 = t2594 * t1494;
    let t10011 = t10010 * t5777;
    let t10013 = t309 * t310 * t1504;
    (t10001, t10003, t10004, t10005, t10011, t10013)
}
