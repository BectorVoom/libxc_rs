//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1255/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1255<F: Float>(t5446: F, t632: F, t10999: F, t11002: F, t11006: F, t11007: F, t11010: F, t11012: F, t11196: F, t11198: F, t11201: F, t11204: F, t11211: F, t14921: F, t14925: F, t14933: F, t14935: F, t14938: F, t14942: F, t14943: F, t14945: F) -> F {
    let t14947 = t5446 * t632;
    let t14948 = F::cast_from(0.5025769232130264_f64) * t14947;
    let t14949 = F::cast_from(0.019897291109174608_f64) * t10999 + F::cast_from(3.9861630686838536_f64) * t11002 + t11006 - F::cast_from(1.7083556008645087_f64) * t14921 + F::cast_from(1.5077307696390791_f64) * t11007 + t11010 + F::cast_from(0.19455129084526285_f64) * t14925 - t11012 - t11196 + F::cast_from(0.5836538725357885_f64) * t11198 - F::cast_from(0.5694518669548363_f64) * t11201 - F::cast_from(13.28721022894618_f64) * t11204 - F::cast_from(0.15917832887339686_f64) * t11211 + t14933 - F::cast_from(4.429070076315393_f64) * t14935 + F::cast_from(0.05969187332752383_f64) * t14938 - t14942 - F::cast_from(0.5025769232130264_f64) * t14943 + F::cast_from(0.2512884616065132_f64) * t14945 + t14948;
    t14949
}
