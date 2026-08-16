//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1232/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1232<F: Float>(t342: F, t4232: F, t4354: F, t301: F, t413: F, t5575: F, t1183: F, t2174: F, t113: F, t395: F, t4463: F, t4394: F, t73: F) -> (F, F, F, F, F) {
    let t14633 = t4232 * t4354 * t342;
    let t14639 = t5575 * t413 * t301;
    let t14640 = F::cast_from(0.0017434044910732151_f64) * t14639;
    let t14642 = t2174 * t1183 * t301;
    let t14646 = t395 * t4463 * t113 * t301;
    let t14648 = t73 * t4394;
    (t14633, t14640, t14642, t14646, t14648)
}
