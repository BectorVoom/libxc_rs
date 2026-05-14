//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 927/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk927<F: Float>(t1059: F, t2993: F, t2998: F, t1010: F, t2735: F, t387: F, t400: F, t1034: F, t40: F, t960: F, t1039: F, t1067: F, t1037: F, t325: F, t333: F, t903: F, t907: F, t935: F) -> (F, F, F, F, F, F, F) {
    let t8482 = t1059 * t2993;
    let t8486 = t1059 * t2998;
    let t8491 = 4.678578717964164 * t400 * t1010 * t2735 * t387;
    let t8493 = t40 * t960 * t1034;
    let t8495 = t1067 * t1039;
    let t8499 = t1067 * t1037;
    let t8505 = 3.436685857643691 * t325 * t903 * t935 * t907 * t333;
    (t8482, t8486, t8491, t8493, t8495, t8499, t8505)
}
