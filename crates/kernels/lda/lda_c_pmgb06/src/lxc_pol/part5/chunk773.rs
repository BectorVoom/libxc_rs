//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 773/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk773<F: Float>(t117: F, t118: F, t123: F, t125: F, t2780: F, t2793: F, t2794: F, t2797: F, t2809: F, t2812: F, t2820: F, t2825: F, t2831: F, t2835: F, t2840: F, t2844: F, t2846: F, t2849: F, t5712: F, t6928: F, t7176: F, t7228: F) -> F {
    let t7236 = -t2780 - t2793 - F::cast_from(0.031505407223141116_f64) * t2794 + t2797 - F::cast_from(0.06301081444628223_f64) * t2809 - t2812 - F::cast_from(0.04789693604101844_f64) * t5712 - F::cast_from(0.031505407223141116_f64) * t6928 * t118 - F::cast_from(0.031505407223141116_f64) * t7176 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * t7228 * t117 + t2820 + t2825 - F::cast_from(0.003950778065781896_f64) * t2831 - F::cast_from(0.0004954275694490498_f64) * t2835 - t2840 - t2844 - t2846 + F::cast_from(0.006584630109636494_f64) * t2849;
    t7236
}
