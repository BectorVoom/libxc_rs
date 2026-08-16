//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1259/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1259(t10492: f64, t10494: f64, t18407: f64, t19131: f64, t199: f64, t22077: f64, t22082: f64, t22084: f64, t22088: f64, t399: f64, t566: f64, t6928: f64, t7375: f64, t7874: f64, t84: f64, t868: f64) -> f64 {
    let t22097 = -0.5694518669548363_f64 * t22077 + 3.9861630686838536_f64 * t18407 + 0.5025769232130264_f64 * t10492 + 0.5025769232130264_f64 * t10494 + 0.2512884616065132_f64 * t22082 + 0.0837628205355044_f64 * t22084 - 0.0837628205355044_f64 * t399 * t7375 - 0.0837628205355044_f64 * t84 * t22088 - 0.0837628205355044_f64 * t19131 * t199 - 0.0837628205355044_f64 * t7874 * t566 - 0.2512884616065132_f64 * t6928 * t868;
    t22097
}
