//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 767/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk767<F: Float>(t2062: F, t5021: F, t830: F, t933: F, t2061: F, t25: F, t4657: F, t4661: F, t4663: F, t4668: F, t4678: F, t4682: F, t4686: F, t4998: F, t5000: F, t5001: F, t5004: F, t5007: F, t5010: F, t5013: F, t5017: F) -> F {
    let t5022 = t5021 * t2062;
    let t5024 = t933 * t830;
    let t5028 = -F::new(0.21595) * t4686 - t4998 + t5000 - F::new(0.0022222222222222222) * t25 * t5001 - F::new(0.002962962962962963) * t25 * t5004 + F::new(0.008888888888888889) * t2061 * t5007 + F::new(0.013333333333333334) * t25 * t5010 - F::new(0.05333333333333334) * t2061 * t5013 - F::new(0.047988888888888886) * t4661 + t5017 - F::new(0.023994444444444443) * t4682 - F::new(0.03999074074074074) * t4668 + F::new(0.09597777777777777) * t4678 - F::new(0.057777777777777775) * t5022 - F::new(0.007407407407407408) * t5024 - F::new(0.015996296296296297) * t4657 - F::new(0.2639388888888889) * t4663;
    t5028
}
