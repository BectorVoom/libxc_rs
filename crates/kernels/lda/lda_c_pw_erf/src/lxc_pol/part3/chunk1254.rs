//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1254/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1254<F: Float>(t14932: F, t153: F, t2869: F, t865: F, t168: F, t5880: F, t635: F, t1210: F, t2292: F, t1896: F, t632: F, t11622: F, t242: F) -> (F, F, F, F, F, F) {
    let t14933 = F::cast_from(3.9861630686838536_f64) * t14932;
    let t14935 = t153 * t2869 * t865;
    let t14938 = t168 * t635 * t5880;
    let t14941 = t168 * t1210 * t2292;
    let t14942 = F::cast_from(0.15917832887339686_f64) * t14941;
    let t14943 = t1896 * t632;
    let t14945 = t11622 * t242;
    (t14933, t14935, t14938, t14942, t14943, t14945)
}
