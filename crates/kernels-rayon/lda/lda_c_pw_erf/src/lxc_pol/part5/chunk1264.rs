//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1264/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1264(t325: f64, t7656: f64, t7636: f64, t7648: f64, t7640: f64, t11866: f64, t11879: f64, t20737: f64, t11867: f64, t11871: f64, t11854: f64, t11855: f64, t11861: f64, t11875: f64, t15836: f64, t17249: f64, t17272: f64, t17274: f64, t17288: f64, t17290: f64, t17295: f64, t17301: f64, t9824: f64, t9847: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22665 = t325 * t7656;
    let t22667 = t325 * t7636;
    let t22669 = t325 * t7648;
    let t22671 = t325 * t7640;
    let t22681 = t11866 * t11879 * t20737;
    let t22684 = t11866 * t11867 * t20737;
    let t22687 = t11866 * t11871 * t20737;
    let t22700 = 0.011997222222222222_f64 * t22665 + 0.013330246913580247_f64 * t22667 + 0.07198333333333333_f64 * t22669 - 0.047988888888888886_f64 * t22671 - 0.10666666666666667_f64 * t17249 + 0.005925925925925926_f64 * t17272 + 0.017777777777777778_f64 * t17274 + 0.08_f64 * t17288 - 0.02666666666666667_f64 * t17290 + 0.0044444444444444444_f64 * t17295 - 0.02666666666666667_f64 * t17301 - 0.64785_f64 * t22681 + 0.4319_f64 * t22684 - 0.11997222222222222_f64 * t22687 - 0.12_f64 * t11854 * t11875 * t20737 + 0.04_f64 * t11854 * t11855 * t20737 - 0.008888888888888889_f64 * t11854 * t11861 * t20737 + t9824 + 0.03732469135802469_f64 * t9847 - 0.14396666666666666_f64 * t15836;
    (t22665, t22667, t22669, t22671, t22681, t22684, t22687, t22700)
}
