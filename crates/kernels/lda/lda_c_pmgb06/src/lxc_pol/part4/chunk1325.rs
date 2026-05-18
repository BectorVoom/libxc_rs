//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1325/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1325<F: Float>(t12494: F, t6633: F, t13053: F, t5138: F, t6629: F, t15275: F, t5139: F, t10134: F, t17384: F, t17386: F, t17389: F, t17392: F, t17395: F, t17398: F, t17402: F, t17407: F, t17410: F, t17414: F, t17416: F) -> (F, F, F, F) {
    let t17418 = F::new(4.0) / F::new(27.0) * t12494 * t6633;
    let t17421 = F::new(4.0) / F::new(27.0) * t5138 * t13053 * t6629;
    let t17424 = F::new(4.0) / F::new(27.0) * t5138 * t5139 * t15275;
    let t17425 = -t17384 + t17386 + t17389 + t17392 + t17395 + t17398 - t17402 + t17407 - t17410 + t17414 - t10134 + t17416 - t17418 - t17421 - t17424;
    (t17418, t17421, t17424, t17425)
}
