//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 898/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk898<F: Float>(t331: F, t5084: F, t5087: F, t5097: F, t5100: F, t2954: F, t739: F, t9777: F, t11: F, t3536: F, t174: F, t4641: F, t9810: F, t11697: F, t11777: F, t1268: F, t25: F, t3516: F, t538: F, t9808: F, t9813: F, t9814: F, t9819: F, t9824: F, t9828: F, t9832: F, t9834: F, t9840: F, t9845: F) -> (F, F, F, F) {
    let t11793 = t331 * t5084;
    let t11798 = t331 * t5087;
    let t11803 = t331 * t5097;
    let t11805 = t331 * t5100;
    let t11808 = t9777 * t739 * t2954;
    let t11813 = t11 * t3536 * t11808;
    let t11818 = t174 * t9810 * t4641;
    let t11825 = 0.023994444444444443 * t9808 + t9813 - 0.02666666666666667 * t11793 - 0.08 * t25 * t1268 * t11697 + 0.08 * t11798 + 0.16 * t25 * t538 * t11777 + 0.0044444444444444444 * t11803 + 0.005925925925925926 * t11805 + 0.035555555555555556 * t25 * t3516 * t11808 + 0.47988888888888886 * t11813 - 0.02666666666666667 * t9814 + 0.0044444444444444444 * t9819 + t9824 + 0.5038833333333333 * t11818 - 0.047988888888888886 * t9828 - 0.03199259259259259 * t9832 + 0.013330246913580247 * t9834 - 0.047988888888888886 * t9840 + 0.011997222222222222 * t9845;
    (t11808, t11813, t11818, t11825)
}
