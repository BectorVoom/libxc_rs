//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1009<F: Float>(t11697: F, t11777: F, t11793: F, t11798: F, t11803: F, t11805: F, t11808: F, t11813: F, t11818: F, t1268: F, t25: F, t3516: F, t538: F, t9808: F, t9813: F, t9814: F, t9819: F, t9824: F, t9828: F, t9832: F, t9834: F, t9840: F, t9845: F) -> F {
    let t11825 = F::cast_from(0.023994444444444443_f64) * t9808 + t9813 - F::cast_from(0.02666666666666667_f64) * t11793 - F::cast_from(0.08_f64) * t25 * t1268 * t11697 + F::cast_from(0.08_f64) * t11798 + F::cast_from(0.16_f64) * t25 * t538 * t11777 + F::cast_from(0.0044444444444444444_f64) * t11803 + F::cast_from(0.005925925925925926_f64) * t11805 + F::cast_from(0.035555555555555556_f64) * t25 * t3516 * t11808 + F::cast_from(0.47988888888888886_f64) * t11813 - F::cast_from(0.02666666666666667_f64) * t9814 + F::cast_from(0.0044444444444444444_f64) * t9819 + t9824 + F::cast_from(0.5038833333333333_f64) * t11818 - F::cast_from(0.047988888888888886_f64) * t9828 - F::cast_from(0.03199259259259259_f64) * t9832 + F::cast_from(0.013330246913580247_f64) * t9834 - F::cast_from(0.047988888888888886_f64) * t9840 + F::cast_from(0.011997222222222222_f64) * t9845;
    t11825
}
