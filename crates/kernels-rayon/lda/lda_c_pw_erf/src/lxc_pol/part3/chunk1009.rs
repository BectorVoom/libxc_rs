//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1009(t11697: f64, t11777: f64, t11793: f64, t11798: f64, t11803: f64, t11805: f64, t11808: f64, t11813: f64, t11818: f64, t1268: f64, t25: f64, t3516: f64, t538: f64, t9808: f64, t9813: f64, t9814: f64, t9819: f64, t9824: f64, t9828: f64, t9832: f64, t9834: f64, t9840: f64, t9845: f64) -> f64 {
    let t11825 = 0.023994444444444443_f64 * t9808 + t9813 - 0.02666666666666667_f64 * t11793 - 0.08_f64 * t25 * t1268 * t11697 + 0.08_f64 * t11798 + 0.16_f64 * t25 * t538 * t11777 + 0.0044444444444444444_f64 * t11803 + 0.005925925925925926_f64 * t11805 + 0.035555555555555556_f64 * t25 * t3516 * t11808 + 0.47988888888888886_f64 * t11813 - 0.02666666666666667_f64 * t9814 + 0.0044444444444444444_f64 * t9819 + t9824 + 0.5038833333333333_f64 * t11818 - 0.047988888888888886_f64 * t9828 - 0.03199259259259259_f64 * t9832 + 0.013330246913580247_f64 * t9834 - 0.047988888888888886_f64 * t9840 + 0.011997222222222222_f64 * t9845;
    t11825
}
