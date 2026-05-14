//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 880/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk880<F: Float>(t1125: F, t153: F, t1891: F, t2869: F, t865: F, t1210: F, t168: F, t2292: F, t1896: F, t632: F, t5446: F, t1143: F, t1901: F, t1905: F, t2929: F, t781: F) -> (F, F, F, F, F, F, F, F) {
    let t14932 = t153 * t1125 * t1891;
    let t14933 = 3.9861630686838536 * t14932;
    let t14935 = t153 * t2869 * t865;
    let t14941 = t168 * t1210 * t2292;
    let t14942 = 0.15917832887339686 * t14941;
    let t14943 = t1896 * t632;
    let t14947 = t5446 * t632;
    let t14948 = 0.5025769232130264 * t14947;
    let t14950 = t1901 * t1143;
    let t14956 = t1905 * t1143;
    let t14957 = 0.2512884616065132 * t14956;
    let t14958 = t781 * t2929;
    (t14933, t14935, t14942, t14943, t14948, t14950, t14957, t14958)
}
