//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 645/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk645<F: Float>(t2092: F, t331: F, t2089: F, t4602: F, t1251: F, t4722: F, t1458: F, t197: F, t1245: F, t1333: F, t4574: F, t1484: F, t219: F) -> (F, F, F, F, F, F, F, F) {
    let t5093 = F::cast_from(0.017777777777777778_f64) * t331 * t2092;
    let t5096 = F::cast_from(0.002962962962962963_f64) * t331 * t2089;
    let t5112 = F::cast_from(0.015996296296296297_f64) * t4602;
    let t5141 = t4722 * t1251;
    let t5146 = t1458 * t197;
    let t5147 = t5146 * t1245;
    let t5160 = t4574 * t1333;
    let t5165 = t1484 * t219;
    (t5093, t5096, t5112, t5141, t5146, t5147, t5160, t5165)
}
