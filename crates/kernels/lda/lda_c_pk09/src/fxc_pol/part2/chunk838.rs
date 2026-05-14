//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 838/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk838<F: Float>(t1475: F, t2508: F, t1349: F, t9920: F, t1337: F, t5279: F, t9946: F, t1348: F, t409: F, t9927: F, t6041: F, t5785: F, t9578: F, t15: F, t309: F, t9581: F) -> (F, F, F, F, F, F, F) {
    let t9989 = t2508 * t1475;
    let t9994 = t1349 * t9920;
    let t9995 = t1337 * t9994;
    let t9997 = t5279 * t9946;
    let t9998 = t1348 * t9997;
    let t10000 = t409 * t9927;
    let t10001 = t6041 * t10000;
    let t10003 = t9578 * t5785;
    let t10004 = t309 * t15;
    let t10005 = t10004 * t9581;
    (t9989, t9995, t9998, t10001, t10003, t10004, t10005)
}
