//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 855/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk855<F: Float>(t11069: F, t11081: F, t11085: F, t11104: F, t11126: F, t11137: F, t11159: F, t11185: F, t26: F, t5939: F, t1295: F, t2236: F, t2229: F, t3588: F, t38: F, t1234: F, t2233: F) -> (F, F, F, F, F) {
    let t11188 = t11069 + t11081 + t11085 + t11104 + t11126 + t11137 + t11159 + t11185;
    let t11200 = t5939 * t26;
    let t11206 = t2236 * t1295;
    let t11211 = 70.1526 * t38 * t2229 * t3588;
    let t11222 = 52.61445 * t38 * t2233 * t1234;
    (t11188, t11200, t11206, t11211, t11222)
}
