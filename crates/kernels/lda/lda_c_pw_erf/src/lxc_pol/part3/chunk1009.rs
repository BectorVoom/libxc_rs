//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1009<F: Float>(t11697: F, t11777: F, t11793: F, t11798: F, t11803: F, t11805: F, t11808: F, t11813: F, t11818: F, t1268: F, t25: F, t3516: F, t538: F, t9808: F, t9813: F, t9814: F, t9819: F, t9824: F, t9828: F, t9832: F, t9834: F, t9840: F, t9845: F) -> F {
    let t11825 = F::new(0.023994444444444443) * t9808 + t9813 - F::new(0.02666666666666667) * t11793 - F::new(0.08) * t25 * t1268 * t11697 + F::new(0.08) * t11798 + F::new(0.16) * t25 * t538 * t11777 + F::new(0.0044444444444444444) * t11803 + F::new(0.005925925925925926) * t11805 + F::new(0.035555555555555556) * t25 * t3516 * t11808 + F::new(0.47988888888888886) * t11813 - F::new(0.02666666666666667) * t9814 + F::new(0.0044444444444444444) * t9819 + t9824 + F::new(0.5038833333333333) * t11818 - F::new(0.047988888888888886) * t9828 - F::new(0.03199259259259259) * t9832 + F::new(0.013330246913580247) * t9834 - F::new(0.047988888888888886) * t9840 + F::new(0.011997222222222222) * t9845;
    t11825
}
