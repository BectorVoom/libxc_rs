//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 882/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk882<F: Float>(t117: F, t118: F, t123: F, t125: F, t2780: F, t2793: F, t2794: F, t2797: F, t2809: F, t2812: F, t2820: F, t2825: F, t2831: F, t2835: F, t2840: F, t2844: F, t2846: F, t2849: F, t5712: F, t6928: F, t7176: F, t7228: F) -> (F,) {
    let t7236 = -t2780 - t2793 - 0.031505407223141116 * t2794 + t2797 - 0.06301081444628223 * t2809 - t2812 - 0.04789693604101844 * t5712 - 0.031505407223141116 * t6928 * t118 - 0.031505407223141116 * t7176 - 0.005388405304614574 * t123 * t125 * t7228 * t117 + t2820 + t2825 - 0.003950778065781896 * t2831 - 0.0004954275694490498 * t2835 - t2840 - t2844 - t2846 + 0.006584630109636494 * t2849;
    (t7236,)
}
