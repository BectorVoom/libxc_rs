//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1082/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1082<F: Float>(t835: F, t9266: F, t1977: F, t3223: F, t11862: F, t160: F, t1983: F, t11903: F, t5137: F, t1414: F, t1639: F, t27: F, t34: F) -> (F, F, F, F, F, F) {
    let t12460 = t9266 * t835;
    let t12462 = t3223 * t1977;
    let t12465 = t160 * t11862 * t1983;
    let t12494 = t11903 * t5137;
    let t12497 = t1639 * t1414;
    let t12514 = t27 * t34;
    (t12460, t12462, t12465, t12494, t12497, t12514)
}
