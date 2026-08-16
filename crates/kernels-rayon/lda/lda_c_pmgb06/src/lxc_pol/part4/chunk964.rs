//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 964/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk964(t117: f64, t118: f64, t123: f64, t125: f64, t2780: f64, t2793: f64, t2794: f64, t2797: f64, t2809: f64, t2812: f64, t2820: f64, t2825: f64, t2831: f64, t2835: f64, t2840: f64, t2844: f64, t2846: f64, t2849: f64, t5712: f64, t6928: f64, t7176: f64, t7228: f64) -> f64 {
    let t7236 = -t2780 - t2793 - 0.031505407223141116_f64 * t2794 + t2797 - 0.06301081444628223_f64 * t2809 - t2812 - 0.04789693604101844_f64 * t5712 - 0.031505407223141116_f64 * t6928 * t118 - 0.031505407223141116_f64 * t7176 - 0.005388405304614574_f64 * t123 * t125 * t7228 * t117 + t2820 + t2825 - 0.003950778065781896_f64 * t2831 - 0.0004954275694490498_f64 * t2835 - t2840 - t2844 - t2846 + 0.006584630109636494_f64 * t2849;
    t7236
}
