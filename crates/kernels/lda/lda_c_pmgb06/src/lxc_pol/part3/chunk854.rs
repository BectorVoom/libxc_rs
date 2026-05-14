//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk854<F: Float>(t2148: F, t3705: F, t8846: F, t8850: F, t11161: F, t11162: F, t11165: F, t11166: F, t11169: F, t11171: F, t11175: F, t11177: F, t11178: F, t8837: F, t8841: F, t8844: F, t8853: F, t9037: F) -> (F,) {
    let t11180 = t2148 * t3705;
    let t11183 = 144.0 * t8846;
    let t11184 = 8.0 * t8850;
    let t11185 = t11161 + 103.89515463408878 * t11162 - t11165 - 1025.4018858216407 * t11166 - t11169 - 1.7544670867903938 * t11171 + t11175 + t11177 - t8837 + t8841 - 0.5848223622634646 * t11178 - 3.5089341735807875 * t11180 - 72.0 * t8844 + t11183 + t11184 - t8853 + t9037;
    (t11185,)
}
