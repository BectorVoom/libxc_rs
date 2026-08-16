//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1052/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1052<F: Float>(t1065: F, t2142: F, t248: F, t3890: F, t897: F, t2148: F, t3760: F, t3705: F, t26: F, t5939: F, t1295: F, t2236: F) -> (F, F, F, F, F, F) {
    let t11174 = t248 * t2142 * t1065;
    let t11177 = t248 * t897 * t3890;
    let t11178 = t2148 * t3760;
    let t11180 = t2148 * t3705;
    let t11200 = t5939 * t26;
    let t11206 = t2236 * t1295;
    (t11174, t11177, t11178, t11180, t11200, t11206)
}
