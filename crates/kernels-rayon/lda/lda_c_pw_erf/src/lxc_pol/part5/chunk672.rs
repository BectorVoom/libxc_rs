//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 672/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk672(t483: f64, t5931: f64, t1187: f64, t169: f64, t1891: f64, t301: f64, t717: f64, t19: f64, t2316: f64, t729: f64, t734: f64, t2343: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5932 = t5931 * t483;
    let t5933 = t5932 * t1187;
    let t5941 = 0.10809180959278285_f64 * t169 * t717 * t1891 * t301;
    let t5949 = t2316 * t729 * t19;
    let t5950 = t5949 * t734;
    let t5967 = t2343 * t75;
    (t5932, t5933, t5941, t5949, t5950, t5967)
}
