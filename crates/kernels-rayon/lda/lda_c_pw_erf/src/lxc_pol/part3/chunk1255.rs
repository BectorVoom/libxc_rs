//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1255/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1255(t5446: f64, t632: f64, t10999: f64, t11002: f64, t11006: f64, t11007: f64, t11010: f64, t11012: f64, t11196: f64, t11198: f64, t11201: f64, t11204: f64, t11211: f64, t14921: f64, t14925: f64, t14933: f64, t14935: f64, t14938: f64, t14942: f64, t14943: f64, t14945: f64) -> f64 {
    let t14947 = t5446 * t632;
    let t14948 = 0.5025769232130264_f64 * t14947;
    let t14949 = 0.019897291109174608_f64 * t10999 + 3.9861630686838536_f64 * t11002 + t11006 - 1.7083556008645087_f64 * t14921 + 1.5077307696390791_f64 * t11007 + t11010 + 0.19455129084526285_f64 * t14925 - t11012 - t11196 + 0.5836538725357885_f64 * t11198 - 0.5694518669548363_f64 * t11201 - 13.28721022894618_f64 * t11204 - 0.15917832887339686_f64 * t11211 + t14933 - 4.429070076315393_f64 * t14935 + 0.05969187332752383_f64 * t14938 - t14942 - 0.5025769232130264_f64 * t14943 + 0.2512884616065132_f64 * t14945 + t14948;
    t14949
}
