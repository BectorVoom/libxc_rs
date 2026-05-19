//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 954/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk954<F: Float>(t11174: F, t248: F, t3890: F, t897: F, t2148: F, t3760: F, t3705: F, t8846: F, t8850: F, t11161: F, t11162: F, t11165: F, t11166: F, t11169: F, t11171: F, t8837: F, t8841: F, t8844: F, t8853: F, t9037: F) -> F {
    let t11175 = F::new(3.0) * t11174;
    let t11177 = t248 * t897 * t3890;
    let t11178 = t2148 * t3760;
    let t11180 = t2148 * t3705;
    let t11183 = F::new(144.0) * t8846;
    let t11184 = F::new(8.0) * t8850;
    let t11185 = t11161 + F::cast_from(103.89515463408878_f64) * t11162 - t11165 - F::cast_from(1025.4018858216407_f64) * t11166 - t11169 - F::cast_from(1.7544670867903938_f64) * t11171 + t11175 + t11177 - t8837 + t8841 - F::cast_from(0.5848223622634646_f64) * t11178 - F::cast_from(3.5089341735807875_f64) * t11180 - F::new(72.0) * t8844 + t11183 + t11184 - t8853 + t9037;
    t11185
}
