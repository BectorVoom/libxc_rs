//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1264/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1264<F: Float>(t325: F, t7656: F, t7636: F, t7648: F, t7640: F, t11866: F, t11879: F, t20737: F, t11867: F, t11871: F, t11854: F, t11855: F, t11861: F, t11875: F, t15836: F, t17249: F, t17272: F, t17274: F, t17288: F, t17290: F, t17295: F, t17301: F, t9824: F, t9847: F) -> (F, F, F, F, F, F, F, F) {
    let t22665 = t325 * t7656;
    let t22667 = t325 * t7636;
    let t22669 = t325 * t7648;
    let t22671 = t325 * t7640;
    let t22681 = t11866 * t11879 * t20737;
    let t22684 = t11866 * t11867 * t20737;
    let t22687 = t11866 * t11871 * t20737;
    let t22700 = F::new(0.011997222222222222) * t22665 + F::new(0.013330246913580247) * t22667 + F::new(0.07198333333333333) * t22669 - F::new(0.047988888888888886) * t22671 - F::new(0.10666666666666667) * t17249 + F::new(0.005925925925925926) * t17272 + F::new(0.017777777777777778) * t17274 + F::new(0.08) * t17288 - F::new(0.02666666666666667) * t17290 + F::new(0.0044444444444444444) * t17295 - F::new(0.02666666666666667) * t17301 - F::new(0.64785) * t22681 + F::new(0.4319) * t22684 - F::new(0.11997222222222222) * t22687 - F::new(0.12) * t11854 * t11875 * t20737 + F::new(0.04) * t11854 * t11855 * t20737 - F::new(0.008888888888888889) * t11854 * t11861 * t20737 + t9824 + F::new(0.03732469135802469) * t9847 - F::new(0.14396666666666666) * t15836;
    (t22665, t22667, t22669, t22671, t22681, t22684, t22687, t22700)
}
