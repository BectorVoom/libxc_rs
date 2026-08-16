//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 954/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk954(t11174: f64, t248: f64, t3890: f64, t897: f64, t2148: f64, t3760: f64, t3705: f64, t8846: f64, t8850: f64, t11161: f64, t11162: f64, t11165: f64, t11166: f64, t11169: f64, t11171: f64, t8837: f64, t8841: f64, t8844: f64, t8853: f64, t9037: f64) -> f64 {
    let t11175 = 3.0_f64 * t11174;
    let t11177 = t248 * t897 * t3890;
    let t11178 = t2148 * t3760;
    let t11180 = t2148 * t3705;
    let t11183 = 144.0_f64 * t8846;
    let t11184 = 8.0_f64 * t8850;
    let t11185 = t11161 + 103.89515463408878_f64 * t11162 - t11165 - 1025.4018858216407_f64 * t11166 - t11169 - 1.7544670867903938_f64 * t11171 + t11175 + t11177 - t8837 + t8841 - 0.5848223622634646_f64 * t11178 - 3.5089341735807875_f64 * t11180 - 72.0_f64 * t8844 + t11183 + t11184 - t8853 + t9037;
    t11185
}
