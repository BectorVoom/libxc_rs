//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 819/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk819<F: Float>(t117: F, t123: F, t740: F, t859: F, t2780: F, t2793: F, t2794: F, t2797: F, t2807: F, t2809: F, t2812: F, t2816: F, t2820: F, t2825: F, t2828: F, t2831: F, t2835: F, t2840: F, t2844: F, t2846: F, t2849: F) -> F {
    let t5712 = t123 * t740 * t859 * t117;
    let t5718 = F::cast_from(0.12602162889256446_f64) * t2816 - t2780 - t2793 - F::cast_from(0.06301081444628223_f64) * t2794 + t2797 - F::cast_from(0.031505407223141116_f64) * t2807 - F::cast_from(0.12602162889256446_f64) * t2809 - t2812 - F::cast_from(0.02394846802050922_f64) * t5712 + t2820 + t2825 - F::cast_from(0.001975389032890948_f64) * t2828 - F::cast_from(0.007901556131563792_f64) * t2831 - F::cast_from(0.0009908551388980995_f64) * t2835 - t2840 - t2844 - t2846 + F::cast_from(0.013169260219272987_f64) * t2849;
    t5718
}
