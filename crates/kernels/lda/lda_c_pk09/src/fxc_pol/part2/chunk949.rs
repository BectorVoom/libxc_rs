//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 949/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk949<F: Float>(t1468: F, t2507: F, t1387: F, t1472: F, t1475: F, t2508: F, t1349: F, t9920: F, t1337: F, t5279: F, t9946: F, t1348: F) -> (F, F, F, F, F) {
    let t9985 = t2507 * t1468;
    let t9986 = t9985 * t1387;
    let t9987 = t9986 * t1472;
    let t9989 = t2508 * t1475;
    let t9994 = t1349 * t9920;
    let t9995 = t1337 * t9994;
    let t9997 = t5279 * t9946;
    let t9998 = t1348 * t9997;
    (t9986, t9987, t9989, t9995, t9998)
}
