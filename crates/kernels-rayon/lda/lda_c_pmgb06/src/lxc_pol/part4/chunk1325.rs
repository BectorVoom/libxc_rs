//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1325/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1325(t12494: f64, t6633: f64, t13053: f64, t5138: f64, t6629: f64, t15275: f64, t5139: f64, t10134: f64, t17384: f64, t17386: f64, t17389: f64, t17392: f64, t17395: f64, t17398: f64, t17402: f64, t17407: f64, t17410: f64, t17414: f64, t17416: f64) -> (f64, f64, f64, f64) {
    let t17418 = 4.0_f64 / 27.0_f64 * t12494 * t6633;
    let t17421 = 4.0_f64 / 27.0_f64 * t5138 * t13053 * t6629;
    let t17424 = 4.0_f64 / 27.0_f64 * t5138 * t5139 * t15275;
    let t17425 = -t17384 + t17386 + t17389 + t17392 + t17395 + t17398 - t17402 + t17407 - t17410 + t17414 - t10134 + t17416 - t17418 - t17421 - t17424;
    (t17418, t17421, t17424, t17425)
}
