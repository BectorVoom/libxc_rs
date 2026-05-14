//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1049/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1049<F: Float>(t168: F, t5880: F, t635: F, t1210: F, t2292: F, t1896: F, t632: F, t11622: F, t242: F, t5446: F, t1143: F, t1901: F, t5451: F, t1905: F, t2929: F, t781: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14938 = t168 * t635 * t5880;
    let t14941 = t168 * t1210 * t2292;
    let t14943 = t1896 * t632;
    let t14945 = t11622 * t242;
    let t14947 = t5446 * t632;
    let t14950 = t1901 * t1143;
    let t14954 = t5451 * t632;
    let t14956 = t1905 * t1143;
    let t14958 = t781 * t2929;
    (t14938, t14941, t14943, t14945, t14947, t14950, t14954, t14956, t14958)
}
