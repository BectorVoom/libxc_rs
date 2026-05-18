//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 772/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk772<F: Float>(t4628: F, t538: F, t2092: F, t331: F, t25: F, t3472: F, t3473: F, t3493: F, t3508: F, t3510: F, t3512: F, t3543: F, t4600: F, t4604: F, t4607: F, t4617: F, t4630: F, t5072: F, t5076: F, t5084: F) -> (F, F) {
    let t5087 = t538 * t4628;
    let t5093 = F::new(0.017777777777777778) * t331 * t2092;
    let t5094 = F::new(0.057777777777777775) * t5072 - F::new(0.015996296296296297) * t4600 + F::new(0.2639388888888889) * t4607 - F::new(0.007407407407407408) * t5076 - t3472 - t3543 - F::new(0.008888888888888889) * t3473 - F::new(0.023994444444444443) * t3493 - F::new(0.014814814814814815) * t3508 + F::new(0.0044444444444444444) * t3510 + F::new(0.0014814814814814814) * t3512 - F::new(0.047988888888888886) * t4604 + F::new(0.013333333333333334) * t25 * t5084 - F::new(0.04) * t25 * t5087 - F::new(0.21595) * t4630 + F::new(0.14396666666666666) * t4617 - t5093;
    (t5087, t5094)
}
