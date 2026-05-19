//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 672/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk672<F: Float>(t483: F, t5931: F, t1187: F, t169: F, t1891: F, t301: F, t717: F, t19: F, t2316: F, t729: F, t734: F, t2343: F, t75: F) -> (F, F, F, F, F, F) {
    let t5932 = t5931 * t483;
    let t5933 = t5932 * t1187;
    let t5941 = F::cast_from(0.10809180959278285_f64) * t169 * t717 * t1891 * t301;
    let t5949 = t2316 * t729 * t19;
    let t5950 = t5949 * t734;
    let t5967 = t2343 * t75;
    (t5932, t5933, t5941, t5949, t5950, t5967)
}
