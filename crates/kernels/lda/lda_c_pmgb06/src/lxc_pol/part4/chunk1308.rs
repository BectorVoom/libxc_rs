//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1308/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1308<F: Float>(t103: F, t16905: F, t16910: F, t17162: F, t17164: F, t17166: F, t17169: F, t17172: F, t17175: F, t17177: F, t17185: F, t17190: F, t17193: F, t3358: F, t9967: F) -> F {
    let t17195 = F::cast_from(0.07198333333333333_f64) * t17162 + F::cast_from(0.026660493827160493_f64) * t17164 - F::cast_from(0.3519185185185185_f64) * t17166 - F::cast_from(0.03999074074074074_f64) * t17169 - F::cast_from(0.10664197530864197_f64) * t17172 + F::cast_from(0.14396666666666666_f64) * t17175 + F::cast_from(0.14396666666666666_f64) * t17177 - F::cast_from(0.002962962962962963_f64) * t103 * t3358 * t16905 - F::cast_from(0.006913580246913581_f64) * t103 * t9967 * t16910 - F::cast_from(0.017777777777777778_f64) * t17185 + F::cast_from(0.14396666666666666_f64) * t17190 - F::new(0.21595) * t17193;
    t17195
}
